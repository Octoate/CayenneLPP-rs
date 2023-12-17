use cayenne_lpp::*;

#[test]
fn api_integration() {
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