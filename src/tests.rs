use super::*;

#[test]
fn reset_ok() {
    let mut buffer: [u8; LPP_DIGITAL_INPUT_SIZE] = [0; LPP_DIGITAL_INPUT_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_digital_input(3, 0x55).unwrap();

    // check slice length (1 digital input)
    let slice_length = lpp.payload_slice().len();
    assert_eq!(3, slice_length);

    // call reset
    lpp.reset();

    // verify that the slice is empty, now
    let slice_length = lpp.payload_slice().len();
    assert_eq!(0, slice_length);

    // just to be sure, check that adding still works
    lpp.add_digital_input(5, 0xAA).unwrap();

    // check slice length (1 digital input)
    let slice_length = lpp.payload_slice().len();
    assert_eq!(3, slice_length);
}

#[test]
fn add_digital_input_ok() {
    let mut buffer: [u8; 2 * LPP_DIGITAL_INPUT_SIZE] = [0; 2 * LPP_DIGITAL_INPUT_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_digital_input(3, 0x55).unwrap();
    lpp.add_digital_input(5, 0xAA).unwrap();

    let expected_bytes: [u8; 6] = [0x03, LPP_DIGITAL_INPUT, 0x55, 0x05, LPP_DIGITAL_INPUT, 0xAA];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_digital_input_overflow() {
    let mut buffer: [u8; LPP_DIGITAL_INPUT_SIZE + 2] = [0; LPP_DIGITAL_INPUT_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_digital_input(3, 0xAA).unwrap();
    let result = lpp.add_digital_input(5, 0x55);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_analog_input() {
    let mut buffer: [u8; 2 * LPP_ANALOG_INPUT_SIZE] = [0; 2 * LPP_ANALOG_INPUT_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_analog_input(3, 27.2).unwrap();
    lpp.add_analog_input(5, 25.5).unwrap();

    let expected_bytes: [u8; 8] = [0x03, LPP_ANALOG_INPUT, 0x0A, 0xA0, 0x05, LPP_ANALOG_INPUT, 0x09, 0xF6];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_analog_input_overflow() {
    let mut buffer: [u8; LPP_ANALOG_INPUT_SIZE + 2] = [0; LPP_ANALOG_INPUT_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_analog_input(3, 27.2).unwrap();
    let result = lpp.add_temperature(5, 25.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_generic_sensor() {
    let mut buffer = [0u8; 2 * LPP_GENERIC_SENSOR_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_generic_sensor(3, 0x01234567).unwrap();
    lpp.add_generic_sensor(5, 0x89abcdef).unwrap();

    let expected_bytes = [
        0x03, LPP_GENERIC_SENSOR, 0x01, 0x23, 0x45, 0x67,
        0x05, LPP_GENERIC_SENSOR, 0x89, 0xab, 0xcd, 0xef];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_generic_sensor_overflow() {
    let mut buffer = [0u8; LPP_GENERIC_SENSOR_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_generic_sensor(3, 123).unwrap();
    let result = lpp.add_generic_sensor(5, 456);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_analog_output() {
    let mut buffer: [u8; 2 * LPP_ANALOG_OUTPUT_SIZE] = [0; 2 * LPP_ANALOG_OUTPUT_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_analog_output(3, 27.2).unwrap();
    lpp.add_analog_output(5, 25.5).unwrap();

    let expected_bytes: [u8; 8] = [0x03, LPP_ANALOG_OUTPUT, 0x0A, 0xA0, 0x05, LPP_ANALOG_OUTPUT, 0x09, 0xF6];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_analog_output_overflow() {
    let mut buffer: [u8; LPP_ANALOG_OUTPUT_SIZE + 2] = [0; LPP_ANALOG_OUTPUT_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_analog_input(3, 27.2).unwrap();
    let result = lpp.add_temperature(5, 25.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_digital_output_ok() {
    let mut buffer: [u8; 2 * LPP_DIGITAL_OUTPUT_SIZE] = [0; 2 * LPP_DIGITAL_OUTPUT_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_digital_output(3, 0x55).unwrap();
    lpp.add_digital_output(5, 0xAA).unwrap();

    let expected_bytes: [u8; 6] = [0x03, LPP_DIGITAL_OUTPUT, 0x55, 0x05, LPP_DIGITAL_OUTPUT, 0xAA];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_digital_output_overflow() {
    let mut buffer: [u8; LPP_DIGITAL_OUTPUT_SIZE + 2] = [0; LPP_DIGITAL_OUTPUT_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_digital_output(3, 0xAA).unwrap();
    let result = lpp.add_digital_output(5, 0x55);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_luminosity_ok() {
    let mut buffer: [u8; 2 * LPP_LUMINOSITY_SIZE] = [0; 2 * LPP_LUMINOSITY_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_luminosity(2, 0xAA55).unwrap();
    lpp.add_luminosity(9, 0x55AA).unwrap();

    let expected_bytes: [u8; 8] = [0x02, LPP_LUMINOSITY, 0xAA, 0x55, 0x09, LPP_LUMINOSITY, 0x55, 0xAA];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_luminosity_overflow() {
    let mut buffer: [u8; LPP_LUMINOSITY_SIZE + 2] = [0; LPP_LUMINOSITY_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_luminosity(2, 0x55AA).unwrap();
    let result = lpp.add_luminosity(5, 0xAA55);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_presence_ok() {
    let mut buffer: [u8; 2 * LPP_PRESENCE_SIZE] = [0; 2 * LPP_PRESENCE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_presence(2, 0xAA).unwrap();
    lpp.add_presence(9, 0x55).unwrap();

    let expected_bytes: [u8; 6] = [0x02, LPP_PRESENCE, 0xAA, 0x09, LPP_PRESENCE, 0x55];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_presence_overflow() {
    let mut buffer: [u8; LPP_PRESENCE_SIZE + 2] = [0; LPP_PRESENCE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_presence(2, 0x55).unwrap();
    let result = lpp.add_presence(5, 0xAA);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_temperature_ok() {
    let mut buffer: [u8; 2 * LPP_TEMPERATURE_SIZE] = [0; 2 * LPP_TEMPERATURE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_temperature(3, 27.2).unwrap();
    lpp.add_temperature(5, 25.5).unwrap();

    let expected_bytes: [u8; 8] = [0x03, LPP_TEMPERATURE, 0x01, 0x10, 0x05, LPP_TEMPERATURE, 0x00, 0xFF];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_temperature_negative_ok() {
    let mut buffer: [u8; 2 * LPP_TEMPERATURE_SIZE] = [0; 2 * LPP_TEMPERATURE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_temperature(6, -12.3).unwrap();
    lpp.add_temperature(2, -35.8).unwrap();

    let expected_bytes: [u8; 8] = [0x06, LPP_TEMPERATURE, 0xFF, 0x85, 0x02, LPP_TEMPERATURE, 0xFE, 0x9A];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_temperature_overflow() {
    let mut buffer: [u8; LPP_TEMPERATURE_SIZE + 2] = [0; LPP_TEMPERATURE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_temperature(3, 27.2).unwrap();
    let result = lpp.add_temperature(5, 25.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_relative_humidity_ok() {
    let mut buffer: [u8; 2 * LPP_RELATIVE_HUMIDITY_SIZE] = [0; 2 * LPP_RELATIVE_HUMIDITY_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_relative_humidity(2, 100.0).unwrap();
    lpp.add_relative_humidity(3, 65.4).unwrap();

    let expected_bytes: [u8; 6] = [0x02, LPP_RELATIVE_HUMIDITY, 0xC8, 0x03, LPP_RELATIVE_HUMIDITY, 0x83];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_relative_humidity_overflow() {
    let mut buffer: [u8; LPP_RELATIVE_HUMIDITY_SIZE + 2] = [0; LPP_RELATIVE_HUMIDITY_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_relative_humidity(3, 27.2).unwrap();
    let result = lpp.add_relative_humidity(5, 25.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_accelerometer_ok() {
    let mut buffer: [u8; 2 * LPP_ACCELEROMETER_SIZE] = [0; 2 * LPP_ACCELEROMETER_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_accelerometer(6, 1.234, -1.234, 0.0).unwrap();
    lpp.add_accelerometer(3, 6.427, 3.129, -2.853).unwrap();

    let expected_bytes: [u8; 16] = [
        0x06, LPP_ACCELEROMETER, 0x04, 0xD2, 0xFB, 0x2E, 0x00, 0x00,
        0x03, LPP_ACCELEROMETER, 0x19, 0x1B, 0x0C, 0x39, 0xF4, 0xDB
    ];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn ass_accelerometer_overflow() {
    let mut buffer: [u8; LPP_ACCELEROMETER_SIZE + 2] = [0; LPP_ACCELEROMETER_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_accelerometer(3, 27.2, 34.2, 56.1).unwrap();
    let result = lpp.add_accelerometer(5, 25.5, 98.1, 23.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_barometric_pressure_ok() {
    let mut buffer: [u8; 2 * LPP_BAROMETRIC_PRESSURE_SIZE] = [0; 2 * LPP_BAROMETRIC_PRESSURE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_barometric_pressure(3, 1023.1).unwrap();
    lpp.add_barometric_pressure(5, 992.3).unwrap();

    let expected_bytes: [u8; 8] = [0x03, LPP_BAROMETRIC_PRESSURE, 0x27, 0xF7, 0x05, LPP_BAROMETRIC_PRESSURE, 0x26, 0xC3];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_barometric_pressure_overflow() {
    let mut buffer: [u8; LPP_BAROMETRIC_PRESSURE_SIZE + 2] = [0; LPP_BAROMETRIC_PRESSURE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_barometric_pressure(3, 27.2).unwrap();
    let result = lpp.add_barometric_pressure(5, 25.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_voltage() {
    let mut buffer = [0u8; LPP_VOLTAGE_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_voltage(3, 123.456).unwrap();
    lpp.add_voltage(5,   7.890).unwrap();

    let expected_bytes = [
        0x03, LPP_VOLTAGE, 0x30, 0x39,
        0x05, LPP_VOLTAGE, 0x03, 0x15
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_voltage_overflow() {
    let mut buffer = [0u8; LPP_VOLTAGE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_voltage(3, 123.456).unwrap();
    let result = lpp.add_voltage(5, 7.89);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_voltage_not_representable() {
    let mut buffer = [0u8; LPP_VOLTAGE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    let result = lpp.add_voltage(1, 789.012);

    assert_eq!(Err(Error::NotRepresentable), result);
}

#[test]
fn add_current() {
    let mut buffer = [0u8; LPP_CURRENT_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_current(3, 12.3456).unwrap();
    lpp.add_current(5,  0.7890).unwrap();

    let expected_bytes = [
        0x03, LPP_CURRENT, 0x30, 0x39,
        0x05, LPP_CURRENT, 0x03, 0x15
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_current_overflow() {
    let mut buffer = [0u8; LPP_CURRENT_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_current(3, 12.3456).unwrap();
    let result = lpp.add_current(5, 0.7890);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_current_not_representable() {
    let mut buffer = [0u8; LPP_VOLTAGE_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    let result = lpp.add_current(1, 789.012);

    assert_eq!(Err(Error::NotRepresentable), result);
}

#[test]
fn add_frequency() {
    let mut buffer = [0u8; LPP_FREQUENCY_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_frequency(3,   910_525_000).unwrap();
    lpp.add_frequency(5, 1_234_567_890).unwrap();

    let expected_bytes = [
        0x03, LPP_FREQUENCY, 0x36, 0x45, 0x82, 0x48,
        0x05, LPP_FREQUENCY, 0x49, 0x96, 0x02, 0xD2,
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_frequency_overflow() {
    let mut buffer = [0u8; LPP_FREQUENCY_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_frequency(3,   910_525_000).unwrap();
    let result = lpp.add_frequency(5, 1_234_567_890);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_percentage() {
    let mut buffer = [0u8; LPP_PERCRENTAGE_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_percentage(3, 12).unwrap();
    lpp.add_percentage(5, 34).unwrap();

    let expected_bytes = [
        3, LPP_PERCRENTAGE, 12,
        5, LPP_PERCRENTAGE, 34
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_percentage_overflow() {
    let mut buffer = [0u8; LPP_PERCRENTAGE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_percentage(3, 12).unwrap();
    let result = lpp.add_percentage(5, 34);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_altitude() {
    let mut buffer = [0u8; LPP_ALTITUDE_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_altitude(3, -1234).unwrap();
    lpp.add_altitude(5,  4567).unwrap();

    let expected_bytes = [
        3, LPP_ALTITUDE, 0xfb, 0x2e,
        5, LPP_ALTITUDE, 0x11, 0xd7
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_altitude_overflow() {
    let mut buffer = [0u8; LPP_ALTITUDE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_altitude(3, -1234).unwrap();
    let result = lpp.add_altitude(5,  4567);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_power() {
    let mut buffer = [0u8; LPP_POWER_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_power(3, 1234).unwrap();
    lpp.add_power(5, 5678).unwrap();

    let expected_bytes = [
        3, LPP_POWER, 0x04, 0xd2,
        5, LPP_POWER, 0x16, 0x2e
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_power_overflow() {
    let mut buffer = [0u8; LPP_POWER_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_power(3, 1234).unwrap();
    let result = lpp.add_power(5, 5678);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_distance() {
    let mut buffer = [0u8; LPP_DISTANCE_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_distance(3, 123456789).unwrap();
    lpp.add_distance(5, 0).unwrap();

    let expected_bytes = [
        3, LPP_DISTANCE, 0x07, 0x5b, 0xcd, 0x15,
        5, LPP_DISTANCE, 0x00, 0x00, 0x00, 0x00
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_distance_overflow() {
    let mut buffer = [0u8; LPP_DISTANCE_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_distance(3, 123456789).unwrap();
    let result = lpp.add_distance(5, 0);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_energy() {
    let mut buffer = [0u8; LPP_ENERGY_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_energy(3, 123456789).unwrap();
    lpp.add_energy(5, 0).unwrap();

    let expected_bytes = [
        3, LPP_ENERGY, 0x07, 0x5b, 0xcd, 0x15,
        5, LPP_ENERGY, 0x00, 0x00, 0x00, 0x00
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_energy_overflow() {
    let mut buffer = [0u8; LPP_ENERGY_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_energy(3, 123456789).unwrap();
    let result = lpp.add_energy(5, 0);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_direction() {
    let mut buffer = [0u8; LPP_DIRECTION_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_direction(3, 123).unwrap();
    lpp.add_direction(5, 359).unwrap();

    let expected_bytes = [
        3, LPP_DIRECTION, 0x00, 0x7b,
        5, LPP_DIRECTION, 0x01, 0x67
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_direction_overflow() {
    let mut buffer = [0u8; LPP_DIRECTION_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_direction(3, 123).unwrap();
    let result = lpp.add_direction(5, 359);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_unixtime() {
    let mut buffer = [0u8; LPP_UNIXTIME_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_unixtime(3, 123456789).unwrap();
    lpp.add_unixtime(5, 0).unwrap();

    let expected_bytes: [u8; 12] = [
        3, LPP_UNIXTIME, 0x07, 0x5b, 0xcd, 0x15,
        5, LPP_UNIXTIME, 0x00, 0x00, 0x00, 0x00
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_unixtime_overflow() {
    let mut buffer = [0u8; LPP_UNIXTIME_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_unixtime(3, 123456789).unwrap();
    let result = lpp.add_unixtime(5, 0);

    assert_eq!(Err(Error::InsufficientMemory), result);
}


#[test]
fn add_gyrometer_ok() {
    let mut buffer: [u8; 2 * LPP_GYROMETER_SIZE] = [0; 2 * LPP_GYROMETER_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_gyrometer(6, 12.34, 56.78, 9.0).unwrap();
    lpp.add_gyrometer(3, 64.27, 31.29, 28.53).unwrap();

    let expected_bytes: [u8; 16] = [
        0x06, LPP_GYROMETER, 0x04, 0xD2, 0x16, 0x2E, 0x03, 0x84,
        0x03, LPP_GYROMETER, 0x19, 0x1A, 0x0C, 0x39, 0x0B, 0x25
    ];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_gyrometer_overflow() {
    let mut buffer: [u8; LPP_GYROMETER_SIZE + 2] = [0; LPP_GYROMETER_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_gyrometer(3, 27.2, 34.2, 56.1).unwrap();
    let result = lpp.add_gyrometer(5, 25.5, 98.1, 23.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_gps_ok() {
    let mut buffer: [u8; 2 * LPP_GPS_SIZE] = [0; 2 * LPP_GPS_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_gps(1, 42.3518, -87.9094, 10.0).unwrap();
    lpp.add_gps(3, 52.3123, 13.2456, 87.65).unwrap();

    let expected_bytes: [u8; 22] = [
        0x01, LPP_GPS, 0x06, 0x76, 0x5E, 0xF2, 0x96, 0x0A, 0x00, 0x03, 0xE8,
        0x03, LPP_GPS, 0x07, 0xFB, 0x73, 0x02, 0x05, 0x68, 0x00, 0x22, 0x3D
    ];
    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_gps_overflow() {
    let mut buffer: [u8; LPP_GPS_SIZE + 2] = [0; LPP_GPS_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_gps(3, 27.2, 34.2, 56.1).unwrap();
    let result = lpp.add_gps(5, 25.5, 98.1, 23.5);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_switch() {
    let mut buffer = [0u8; LPP_SWITCH_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_switch(3, 0).unwrap();
    lpp.add_switch(5, 1).unwrap();

    let expected_bytes = [
        3, LPP_SWITCH, 0,
        5, LPP_SWITCH, 1
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_switch_overflow() {
    let mut buffer = [0u8; LPP_SWITCH_SIZE + 1];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_switch(3, 0).unwrap();
    let result = lpp.add_switch(5, 1);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_concentration() {
    let mut buffer = [0u8; LPP_CONCENTRATION_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_concentration(3, 65_000).unwrap();
    lpp.add_concentration(5, 12_345).unwrap();

    let expected_bytes = [
        3, LPP_CONCENTRATION, 0xfd, 0xe8,
        5, LPP_CONCENTRATION, 0x30, 0x39
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_concentration_overflow() {
    let mut buffer = [0u8; LPP_CONCENTRATION_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_concentration(3, 65_000).unwrap();
    let result = lpp.add_concentration(5, 12_345);

    assert_eq!(Err(Error::InsufficientMemory), result);
}

#[test]
fn add_color() {
    let mut buffer = [0u8; LPP_COLOR_SIZE * 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_color(3, 0x12, 0x34, 0x56).unwrap();
    lpp.add_color(5, 0x78, 0x9a, 0xbc).unwrap();

    let expected_bytes = [
        3, LPP_COLOR, 0x12, 0x34, 0x56,
        5, LPP_COLOR, 0x78, 0x9a, 0xbc
    ];

    assert_eq!(expected_bytes, buffer);
}

#[test]
fn add_color_overflow() {
    let mut buffer = [0u8; LPP_COLOR_SIZE + 2];
    let mut lpp = CayenneLPP::new(&mut buffer);

    lpp.add_color(3, 0x12, 0x34, 0x56).unwrap();
    let result = lpp.add_color(5, 0x78, 0x9a, 0xbc);

    assert_eq!(Err(Error::InsufficientMemory), result);
}