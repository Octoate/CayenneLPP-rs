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
    fn create(buffer: &'a mut [u8]) -> Self {
        CayenneLPP {
            buffer,
            index: 0
        }
    }

    fn add_temperature(&mut self, channel: u8, celsius: f32) -> Result<(), ()> {
        if self.index >= self.buffer.len() {
            return Err(());
        }

        let temperature: u16 = (celsius * 10.0) as u16;
        let temperature_bytes = temperature.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_TEMPERATURE;
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[1];
        self.index += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_temperature() {
        let mut buffer: [u8; 2 * LPP_TEMPERATURE_SIZE] = [0; 2 * LPP_TEMPERATURE_SIZE];
        let mut lpp = CayenneLPP::create(&mut buffer);
        
        lpp.add_temperature(3, 27.2).unwrap();
        lpp.add_temperature(5, 25.5).unwrap();

        let expected_bytes: [u8; 8] = [0x03, 0x67, 0x01, 0x10, 0x05, 0x67, 0x00, 0xFF];
        assert_eq!(expected_bytes, buffer);
    }

    #[test]
    fn add_temperature_overflow() {
        let mut buffer: [u8; LPP_TEMPERATURE_SIZE] = [0; LPP_TEMPERATURE_SIZE];
        let mut lpp = CayenneLPP::create(&mut buffer);
        
        lpp.add_temperature(3, 27.2).unwrap();
        let result = lpp.add_temperature(5, 25.5);

        assert_eq!(Err(()), result);
    }
}
