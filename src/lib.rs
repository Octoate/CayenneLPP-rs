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

    fn add_temperature(&mut self, channel: u8, celsius: f32) {
        let temperature: u16 = (celsius * 10.0) as u16;
        let temperature_bytes = temperature.to_be_bytes();

        self.buffer[{ self.index += 1; self.index }] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_TEMPERATURE;
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = temperature_bytes[1];
    }
}

pub fn add(left: usize, right: usize) -> usize {
    let mut buffer: [u8; 100] = [0; 100];
    let mut lpp = CayenneLPP::create(&mut buffer);
    lpp.add_temperature(0, 32.3);

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
