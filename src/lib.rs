#![no_std]

const LPP_DIGITAL_INPUT: u8 =       0;       // 1 byte
const LPP_DIGITAL_OUTPUT: u8 =      1;       // 1 byte
const LPP_ANALOG_INPUT: u8 =        2;       // 2 bytes, 0.01 signed
const LPP_ANALOG_OUTPUT: u8 =       3;       // 2 bytes, 0.01 signed
const LPP_LUMINOSITY: u8 =          101;     // 2 bytes, 1 lux unsigned
const LPP_PRESENCE: u8 =            102;     // 1 byte, 1
const LPP_TEMPERATURE: u8 =         103;     // 2 bytes, 0.1°C signed
const LPP_RELATIVE_HUMIDITY: u8 =   104;     // 1 byte, 0.5% unsigned
const LPP_ACCELEROMETER: u8 =       113;     // 2 bytes per axis, 0.001G
const LPP_BAROMETRIC_PRESSURE: u8 = 115;     // 2 bytes 0.1 hPa Unsigned
const LPP_GYROMETER: u8 =           134;     // 2 bytes per axis, 0.01 °/s
const LPP_GPS: u8 =                 136;     // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

// Data ID + Data Type + Data Size
const LPP_DIGITAL_INPUT_SIZE: usize =       3;       // 1 byte
const LPP_DIGITAL_OUTPUT_SIZE: usize =      3;       // 1 byte
const LPP_ANALOG_INPUT_SIZE: usize =        4;       // 2 bytes, 0.01 signed
const LPP_ANALOG_OUTPUT_SIZE: usize =       4;       // 2 bytes, 0.01 signed
const LPP_LUMINOSITY_SIZE: usize =          4;       // 2 bytes, 1 lux unsigned
const LPP_PRESENCE_SIZE: usize =            3;       // 1 byte, 1
const LPP_TEMPERATURE_SIZE: usize =         4;       // 2 bytes, 0.1°C signed
const LPP_RELATIVE_HUMIDITY_SIZE: usize =   3;       // 1 byte, 0.5% unsigned
const LPP_ACCELEROMETER_SIZE: usize =       8;       // 2 bytes per axis, 0.001G
const LPP_BAROMETRIC_PRESSURE_SIZE: usize = 4;       // 2 bytes 0.1 hPa Unsigned
const LPP_GYROMETER_SIZE: usize =           8;       // 2 bytes per axis, 0.01 °/s
const LPP_GPS_SIZE: usize =                 11;      // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

struct CayenneLPP<'a> {
    buffer: &'a mut [u8],
    index: usize
}

impl<'a> CayenneLPP<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        CayenneLPP {
            buffer,
            index: 0
        }
    }

    fn add_digital_input(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_DIGITAL_INPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_INPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    fn add_digital_output(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_DIGITAL_OUTPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_OUTPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    fn add_analog_input(&mut self, channel: u8, value: f32) -> Result<(), ()> {
        if self.index + LPP_ANALOG_INPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        let analog_input: i16 = (value * 100.0) as i16;
        let analog_input_bytes = analog_input.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_ANALOG_INPUT;
        self.buffer[{ self.index += 1; self.index }] = analog_input_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = analog_input_bytes[1];
        self.index += 1;

        Ok(())
    }

    fn add_analog_output(&mut self, channel: u8, value: f32) -> Result<(), ()> {
        if self.index + LPP_ANALOG_OUTPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        let analog_output: i16 = (value * 100.0) as i16;
        let analog_output_bytes = analog_output.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_ANALOG_OUTPUT;
        self.buffer[{ self.index += 1; self.index }] = analog_output_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = analog_output_bytes[1];
        self.index += 1;

        Ok(())
    }

    fn add_luminosity(&mut self, channel: u8, lux: u16) -> Result<(), ()> {
        if self.index + LPP_LUMINOSITY_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_LUMINOSITY;
        self.buffer[{ self.index += 1; self.index }] = (lux >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = lux as u8;
        self.index += 1;

        Ok(())
    }

    fn add_presence(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_PRESENCE_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_PRESENCE;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    fn add_temperature(&mut self, channel: u8, celsius: f32) -> Result<(), ()> {
        if self.index + LPP_TEMPERATURE_SIZE > self.buffer.len() {
            return Err(());
        }

        let temperature: i16 = (celsius * 10.0) as i16;
        let temperature_bytes = temperature.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_TEMPERATURE;
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[1];
        self.index += 1;

        Ok(())
    }

    fn add_relative_humidity(&mut self, channel: u8, relative_humidity: f32) -> Result<(), ()> {
        if self.index + LPP_RELATIVE_HUMIDITY_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_RELATIVE_HUMIDITY;
        self.buffer[{ self.index += 1; self.index }] = (relative_humidity * 2.0) as u8;
        self.index += 1;

        Ok(())
    }

    fn add_accelerometer(&mut self, channel: u8, x: f32, y: f32, z: f32) -> Result<(), ()> {
        if self.index + LPP_ACCELEROMETER_SIZE > self.buffer.len() {
            return Err(());
        }

        // prepare axis values
        let vx: i16 = (x * 1000.0) as i16;
        let vy: i16 = (y * 1000.0) as i16;
        let vz: i16 = (z * 1000.0) as i16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_ACCELEROMETER;
        self.buffer[{ self.index += 1; self.index }] = (vx >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vx as u8;
        self.buffer[{ self.index += 1; self.index }] = (vy >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vy as u8;
        self.buffer[{ self.index += 1; self.index }] = (vz >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vz as u8;
        self.index += 1;

        Ok(())
    }

    fn add_barometric_pressure(&mut self, channel: u8, hpa: f32) -> Result<(), ()> {
        if self.index + LPP_BAROMETRIC_PRESSURE_SIZE > self.buffer.len() {
            return Err(());
        }

        let pressure = (hpa * 10.0) as u16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_BAROMETRIC_PRESSURE;
        self.buffer[{ self.index += 1; self.index }] = (pressure >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = pressure as u8;
        self.index += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
    }

    #[test]
    fn add_relative_humidity_ok() {
        let mut buffer: [u8; 2 * LPP_RELATIVE_HUMIDITY_SIZE] = [0; 2 * LPP_RELATIVE_HUMIDITY_SIZE];
        let mut lpp = CayenneLPP::new(&mut buffer);

        lpp.add_relative_humidity(2, 100.0).unwrap();
        lpp.add_relative_humidity(3, 65.4).unwrap();

        let expected_bytes: [u8; 6] = [0x02, LPP_RELATIVE_HUMIDITY, 0xC8, 0x03, LPP_RELATIVE_HUMIDITY, 0x82];
        assert_eq!(expected_bytes, buffer);
    }

    #[test]
    fn add_relative_humidity_overflow() {
        let mut buffer: [u8; LPP_RELATIVE_HUMIDITY_SIZE + 2] = [0; LPP_RELATIVE_HUMIDITY_SIZE + 2];
        let mut lpp = CayenneLPP::new(&mut buffer);

        lpp.add_relative_humidity(3, 27.2).unwrap();
        let result = lpp.add_relative_humidity(5, 25.5);

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
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

        assert_eq!(Err(()), result);
    }
}
