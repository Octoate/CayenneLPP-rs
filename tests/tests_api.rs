use cayenne_lpp::*;

#[test]
fn test_all_possible_payloads() {
    // prepare an array that will fit the whole payload and
    // use a bigger buffer, to check whether the payload slice is correct
    const ADDITIONAL_BYTES: usize = 2;
    let mut buffer = [0u8;
        LPP_DIGITAL_INPUT_SIZE +
        LPP_DIGITAL_OUTPUT_SIZE +
        LPP_ANALOG_INPUT_SIZE +
        LPP_ANALOG_OUTPUT_SIZE +
        LPP_GENERIC_SENSOR_SIZE +
        LPP_LUMINOSITY_SIZE +
        LPP_PRESENCE_SIZE +
        LPP_TEMPERATURE_SIZE +
        LPP_RELATIVE_HUMIDITY_SIZE +
        LPP_ACCELEROMETER_SIZE +
        LPP_BAROMETRIC_PRESSURE_SIZE +
        LPP_VOLTAGE_SIZE +
        LPP_CURRENT_SIZE +
        LPP_FREQUENCY_SIZE +
        LPP_PERCENTAGE_SIZE +
        LPP_ALTITUDE_SIZE +
        LPP_POWER_SIZE +
        LPP_DISTANCE_SIZE +
        LPP_ENERGY_SIZE +
        LPP_DIRECTION_SIZE +
        LPP_UNIXTIME_SIZE +
        LPP_GYROMETER_SIZE +
        LPP_CONCENTRATION_SIZE +
        LPP_COLOR_SIZE +
        LPP_GPS_SIZE +
        LPP_SWITCH_SIZE +
        ADDITIONAL_BYTES
    ];

    let mut lpp = CayenneLPP::new(&mut buffer);

    // add payload for all different values (let's use the same values as in the unit tests for easier testing)
    lpp.add_digital_input(3, 0x55).unwrap();
    lpp.add_digital_output(5, 0xAA).unwrap();
    lpp.add_analog_input(3, 27.2).unwrap();
    lpp.add_analog_output(5, 25.5).unwrap();
    lpp.add_generic_sensor(3, 0x1234567).unwrap();
    lpp.add_luminosity(9, 0x55AA).unwrap();
    lpp.add_presence(2, 0xAA).unwrap();
    lpp.add_temperature(5, 25.5).unwrap();
    lpp.add_relative_humidity(3, 65.4).unwrap();
    lpp.add_accelerometer(3, 6.427, 3.129, -2.853).unwrap();
    lpp.add_barometric_pressure(5, 992.3).unwrap();
    lpp.add_voltage(3, 123.456).unwrap();
    lpp.add_current(5, 12.345).unwrap();
    lpp.add_frequency(3, 910_525_000).unwrap();
    lpp.add_percentage(3, 12).unwrap();
    lpp.add_altitude(3, -1234).unwrap();
    lpp.add_power(5, 1234).unwrap();
    lpp.add_distance(3, 123456789).unwrap();
    lpp.add_energy(5, 123456789).unwrap();
    lpp.add_direction(3, 123).unwrap();
    lpp.add_unixtime(5, 123456789).unwrap();
    lpp.add_gyrometer(6, 12.34, 56.78, 9.0).unwrap();
    lpp.add_gps(1, 42.3518, -87.9094, 10.0).unwrap();
    lpp.add_switch(3, true).unwrap();
    lpp.add_concentration(5, 65000).unwrap();
    lpp.add_color(3, 0x12, 0x34, 0x56).unwrap();

    // create the expected bytes...
    let expected_bytes = [
        0x03, LPP_DIGITAL_INPUT, 0x55,
        0x05, LPP_DIGITAL_OUTPUT, 0xAA,
        0x03, LPP_ANALOG_INPUT, 0x0A, 0xA0,
        0x05, LPP_ANALOG_OUTPUT, 0x09, 0xF6,
        0x03, LPP_GENERIC_SENSOR, 0x01, 0x23, 0x45, 0x67,
        0x09, LPP_LUMINOSITY, 0x55, 0xAA,
        0x02, LPP_PRESENCE, 0xAA,
        0x05, LPP_TEMPERATURE, 0x00, 0xFF,
        0x03, LPP_RELATIVE_HUMIDITY, 0x83,
        0x03, LPP_ACCELEROMETER, 0x19, 0x1B, 0x0C, 0x39, 0xF4, 0xDB,
        0x05, LPP_BAROMETRIC_PRESSURE, 0x26, 0xC3,
        0x03, LPP_VOLTAGE, 0x30, 0x39,
        0x05, LPP_CURRENT, 0x30, 0x39,
        0x03, LPP_FREQUENCY, 0x36, 0x45, 0x82, 0x48,
        0x03, LPP_PERCENTAGE, 12,
        0x03, LPP_ALTITUDE, 0xfb, 0x2e,
        0x05, LPP_POWER, 0x04, 0xd2,
        0x03, LPP_DISTANCE, 0x07, 0x5b, 0xcd, 0x15,
        0x05, LPP_ENERGY, 0x07, 0x5b, 0xcd, 0x15,
        0x03, LPP_DIRECTION, 0x00, 0x7b,
        0x05, LPP_UNIXTIME, 0x07, 0x5b, 0xcd, 0x15,
        0x06, LPP_GYROMETER, 0x04, 0xD2, 0x16, 0x2E, 0x03, 0x84,
        0x01, LPP_GPS, 0x06, 0x76, 0x5E, 0xF2, 0x96, 0x0A, 0x00, 0x03, 0xE8,
        0x03, LPP_SWITCH, 1,
        0x05, LPP_CONCENTRATION, 0xfd, 0xe8,
        0x03, LPP_COLOR, 0x12, 0x34, 0x56,
    ];

    // ...and compare them
    let buffer_slice = lpp.payload_slice();
    assert_eq!(expected_bytes, buffer_slice);
}

#[test]
fn test_scalar_and_iter() {
    // prepare an array that will fit the whole payload
    const ADDITIONAL_BYTES: usize = 2;
    let mut buffer = [0u8;
        LPP_DIGITAL_INPUT_SIZE +
        LPP_DIGITAL_OUTPUT_SIZE +
        LPP_ANALOG_INPUT_SIZE +
        LPP_ANALOG_OUTPUT_SIZE +
        LPP_GENERIC_SENSOR_SIZE +
        LPP_LUMINOSITY_SIZE +
        LPP_PRESENCE_SIZE +
        LPP_TEMPERATURE_SIZE +
        LPP_RELATIVE_HUMIDITY_SIZE +
        LPP_ACCELEROMETER_SIZE +
        LPP_BAROMETRIC_PRESSURE_SIZE +
        LPP_VOLTAGE_SIZE +
        LPP_CURRENT_SIZE +
        LPP_FREQUENCY_SIZE +
        LPP_PERCRENTAGE_SIZE +
        LPP_ALTITUDE_SIZE +
        LPP_POWER_SIZE +
        LPP_DISTANCE_SIZE +
        LPP_ENERGY_SIZE +
        LPP_DIRECTION_SIZE +
        LPP_UNIXTIME_SIZE +
        LPP_GYROMETER_SIZE +
        LPP_CONCENTRATION_SIZE +
        LPP_COLOR_SIZE +
        LPP_GPS_SIZE +
        LPP_SWITCH_SIZE +
        ADDITIONAL_BYTES
    ];

    let mut lpp = CayenneLPP::new(&mut buffer);

    // Make an array of all the possible values of a scalar
    // This will ensure that the code to add a scalar to the
    // data structure is corrrect, and it'll also be used to
    // verify that we can correctly pull them back out via
    // the iterator;
    let scalars = [
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::DigitalInput(0x55) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::DigitalOutput(0xAA) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::AnalogInput(12.7) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::AnalogOutput(15.5) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::GenericSensor( 0x1234567) },
        CayenneLPPScalar{ channel: 9, value: CayenneLPPValue::Luminosity(0x55AA) },
        CayenneLPPScalar{ channel: 2, value: CayenneLPPValue::Presence(0xAA) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::Temperature(25.5) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::RelativeHumidity(65.5) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Accelerometer(6.427, 3.129, -2.853) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::BarometricPressure(992.3) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Voltage(123.45) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::Current(12.345) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Frequency(901_525_000) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Percentage(12) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Altitude(-1234) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::Power(1234) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Distance(123456789) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::Energy(123456789) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Direction(123) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::UnixTime(123456789) },
        CayenneLPPScalar{ channel: 6, value: CayenneLPPValue::Gyrometer(12.34, 56.78, 9.0) },
        CayenneLPPScalar{ channel: 1, value: CayenneLPPValue::GPS(42.3518, -87.9094, 10.0) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Switch(true) },
        CayenneLPPScalar{ channel: 5, value: CayenneLPPValue::Concentration(65000) },
        CayenneLPPScalar{ channel: 3, value: CayenneLPPValue::Color(0x12, 0x34, 0x56) },
    ];

    for scalar in scalars.into_iter() {
        lpp.add_scalar(&scalar).unwrap();
    }

    // Now, we should have all the data in the lpp structure;
    // well iterate through the scalars zip'ed with the sample
    // data and compare to make sure they match.  This will stop
    // when one of the iters completes, so we'll keep track of
    // how many we process.  If it's less than the size of the
    // example array we know we terminated early.
    let mut count = 0;
    for (example, result) in scalars.into_iter().zip(lpp.into_iter()) {
        assert_eq!(example, result);
        count += 1;
    }

    assert_eq!(count, scalars.len());

}