use cayenne_lpp::*;

#[test]
fn test_all_possible_payloads() {
    // prepare an array that will exactly fit the whole payload
    let mut buffer: [u8; LPP_DIGITAL_INPUT_SIZE + LPP_DIGITAL_OUTPUT_SIZE + LPP_ANALOG_INPUT_SIZE +
        LPP_ANALOG_OUTPUT_SIZE + LPP_LUMINOSITY_SIZE + LPP_PRESENCE_SIZE + LPP_TEMPERATURE_SIZE +
        LPP_RELATIVE_HUMIDITY_SIZE + LPP_ACCELEROMETER_SIZE + LPP_BAROMETRIC_PRESSURE_SIZE +
        LPP_GYROMETER_SIZE + LPP_GPS_SIZE] = [0; LPP_DIGITAL_INPUT_SIZE + LPP_DIGITAL_OUTPUT_SIZE +
        LPP_ANALOG_INPUT_SIZE + LPP_ANALOG_OUTPUT_SIZE + LPP_LUMINOSITY_SIZE + LPP_PRESENCE_SIZE +
        LPP_TEMPERATURE_SIZE + LPP_RELATIVE_HUMIDITY_SIZE + LPP_ACCELEROMETER_SIZE +
        LPP_BAROMETRIC_PRESSURE_SIZE + LPP_GYROMETER_SIZE + LPP_GPS_SIZE];
    let mut lpp = CayenneLPP::new(&mut buffer);

    // add payload for all different values (let's use the same values as in the unit tests for easier testing)
    lpp.add_digital_input(3, 0x55).unwrap();
    lpp.add_digital_output(5, 0xAA).unwrap();
    lpp.add_analog_input(3, 27.2).unwrap();
    lpp.add_analog_output(5, 25.5).unwrap();
    lpp.add_luminosity(9, 0x55AA).unwrap();
    lpp.add_presence(2, 0xAA).unwrap();
    lpp.add_temperature(5, 25.5).unwrap();
    lpp.add_relative_humidity(3, 65.4).unwrap();
    lpp.add_accelerometer(3, 6.427, 3.129, -2.853).unwrap();
    lpp.add_barometric_pressure(5, 992.3).unwrap();
    lpp.add_gyrometer(6, 12.34, 56.78, 9.0).unwrap();
    lpp.add_gps(1, 42.3518, -87.9094, 10.0).unwrap();

    // create the expected bytes...
    let expected_bytes: [u8; 59] = [
        0x03, LPP_DIGITAL_INPUT, 0x55,
        0x05, LPP_DIGITAL_OUTPUT, 0xAA,
        0x03, LPP_ANALOG_INPUT, 0x0A, 0xA0,
        0x05, LPP_ANALOG_OUTPUT, 0x09, 0xF6,
        0x09, LPP_LUMINOSITY, 0x55, 0xAA,
        0x02, LPP_PRESENCE, 0xAA,
        0x05, LPP_TEMPERATURE, 0x00, 0xFF,
        0x03, LPP_RELATIVE_HUMIDITY, 0x82,
        0x03, LPP_ACCELEROMETER, 0x19, 0x1B, 0x0C, 0x39, 0xF4, 0xDB,
        0x05, LPP_BAROMETRIC_PRESSURE, 0x26, 0xC3,
        0x06, LPP_GYROMETER, 0x04, 0xD2, 0x16, 0x2E, 0x03, 0x84,
        0x01, LPP_GPS, 0x06, 0x76, 0x5E, 0xF2, 0x96, 0x0A, 0x00, 0x03, 0xE8
    ];
    // ...and compare them
    assert_eq!(expected_bytes, buffer);
}