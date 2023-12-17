#![no_std]
#![deny(missing_docs, unsafe_code, unstable_features, unused_import_braces, unused_qualifications, trivial_casts,
trivial_numeric_casts)]

//! This crate is a port of the [Cayenne LPP] (Low Power Payload) API. It provides an easy way to send data over LPWAN
//! networks such as LoRaWAN. Cayenne LPP is compliant with payload size restrictions, which can be lowered down to
//! 11 bytes and allows the device to send multiple sensor data at one time.
//!
//! Additionally it is also possible to send different sensor data in different frames. To do this, the channel value
//! of the data can be used.
//!
//! The original C++ version of [Cayenne LPP] can be found [here].
//! [Cayenne LPP]: https://docs.mydevices.com/docs/lorawan/cayenne-lpp
//! [here]: https://github.com/myDevicesIoT/CayenneLPP

#[cfg(test)]
mod tests;

/// Data type of a digital input
pub const LPP_DIGITAL_INPUT: u8 =       0;       // 1 byte

/// Data type of a digital output
pub const LPP_DIGITAL_OUTPUT: u8 =      1;       // 1 byte

/// Data type of an analog input
pub const LPP_ANALOG_INPUT: u8 =        2;       // 2 bytes, 0.01 signed

/// Data type of an analog output
pub const LPP_ANALOG_OUTPUT: u8 =       3;       // 2 bytes, 0.01 signed

/// Data type of a luminosity value
pub const LPP_LUMINOSITY: u8 =          101;     // 2 bytes, 1 lux unsigned

// Data type of a presence sensor
pub const LPP_PRESENCE: u8 =            102;     // 1 byte, 1

/// Data type of a temperature value
pub const LPP_TEMPERATURE: u8 =         103;     // 2 bytes, 0.1°C signed

/// Data type of a relative humidity value
pub const LPP_RELATIVE_HUMIDITY: u8 =   104;     // 1 byte, 0.5% unsigned

/// Data type of accelerometer values
pub const LPP_ACCELEROMETER: u8 =       113;     // 2 bytes per axis, 0.001G

/// Data type of a barometric pressure value
pub const LPP_BAROMETRIC_PRESSURE: u8 = 115;     // 2 bytes 0.1 hPa Unsigned

/// Data type of gyrometer values
pub const LPP_GYROMETER: u8 =           134;     // 2 bytes per axis, 0.01 °/s

/// Data type of GPS value
pub const LPP_GPS: u8 =                 136;     // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

// Data ID + Data Type + Data Size
/// Size of a digital input packet including channel and data type
pub const LPP_DIGITAL_INPUT_SIZE: usize =       3;       // 1 byte

/// Size of a digital output packet including channel and data type
pub const LPP_DIGITAL_OUTPUT_SIZE: usize =      3;       // 1 byte

/// Size of an analog input packet including channel and data type
pub const LPP_ANALOG_INPUT_SIZE: usize =        4;       // 2 bytes, 0.01 signed

/// Size of an analog output packet including channel and data type
pub const LPP_ANALOG_OUTPUT_SIZE: usize =       4;       // 2 bytes, 0.01 signed

/// Size of a luminosity packet including channel and data type
pub const LPP_LUMINOSITY_SIZE: usize =          4;       // 2 bytes, 1 lux unsigned

/// Size of a presence sensor packet including channel and data type
pub const LPP_PRESENCE_SIZE: usize =            3;       // 1 byte, 1

/// Size of a temperature packet including channel and data type
pub const LPP_TEMPERATURE_SIZE: usize =         4;       // 2 bytes, 0.1°C signed

/// Size of a relative humidity packet including channel and data type
pub const LPP_RELATIVE_HUMIDITY_SIZE: usize =   3;       // 1 byte, 0.5% unsigned

/// Size of an accelerometer packet including channel and data type
pub const LPP_ACCELEROMETER_SIZE: usize =       8;       // 2 bytes per axis, 0.001G

/// Size of a barometric pressure packet including channel and data type
pub const LPP_BAROMETRIC_PRESSURE_SIZE: usize = 4;       // 2 bytes 0.1 hPa Unsigned

/// Size of a gyrometer packet including channel and data type
pub const LPP_GYROMETER_SIZE: usize =           8;       // 2 bytes per axis, 0.01 °/s

/// Size of a GPS packet including channel and data type
pub const LPP_GPS_SIZE: usize =                 11;      // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

pub struct CayenneLPP<'a> {
    buffer: &'a mut [u8],
    index: usize
}

impl<'a> CayenneLPP<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        CayenneLPP {
            buffer,
            index: 0
        }
    }

    pub fn add_digital_input(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_DIGITAL_INPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_INPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    pub fn add_digital_output(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_DIGITAL_OUTPUT_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_OUTPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    pub fn add_analog_input(&mut self, channel: u8, value: f32) -> Result<(), ()> {
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

    pub fn add_analog_output(&mut self, channel: u8, value: f32) -> Result<(), ()> {
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

    pub fn add_luminosity(&mut self, channel: u8, lux: u16) -> Result<(), ()> {
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

    pub fn add_presence(&mut self, channel: u8, value: u8) -> Result<(), ()> {
        if self.index + LPP_PRESENCE_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_PRESENCE;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    pub fn add_temperature(&mut self, channel: u8, celsius: f32) -> Result<(), ()> {
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

    pub fn add_relative_humidity(&mut self, channel: u8, relative_humidity: f32) -> Result<(), ()> {
        if self.index + LPP_RELATIVE_HUMIDITY_SIZE > self.buffer.len() {
            return Err(());
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_RELATIVE_HUMIDITY;
        self.buffer[{ self.index += 1; self.index }] = (relative_humidity * 2.0) as u8;
        self.index += 1;

        Ok(())
    }

    pub fn add_accelerometer(&mut self, channel: u8, x: f32, y: f32, z: f32) -> Result<(), ()> {
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

    pub fn add_barometric_pressure(&mut self, channel: u8, hpa: f32) -> Result<(), ()> {
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

    pub fn add_gyrometer(&mut self, channel: u8, x: f32, y: f32, z: f32) -> Result<(), ()> {
        if self.index + LPP_GYROMETER_SIZE > self.buffer.len() {
            return Err(());
        }

        // prepare axis values
        let vx: u16 = (x * 100.0) as u16;
        let vy: u16 = (y * 100.0) as u16;
        let vz: u16 = (z * 100.0) as u16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_GYROMETER;
        self.buffer[{ self.index += 1; self.index }] = (vx >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vx as u8;
        self.buffer[{ self.index += 1; self.index }] = (vy >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vy as u8;
        self.buffer[{ self.index += 1; self.index }] = (vz >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vz as u8;
        self.index += 1;

        Ok(())
    }

    pub fn add_gps(&mut self, channel: u8, latitude: f32, longitude: f32, meters: f32) -> Result<(), ()> {
        if self.index + LPP_GPS_SIZE > self.buffer.len() {
            return Err(());
        }

        // prepare GPS values (3 bytes each)
        let vx: i32 = (latitude * 10000.0) as i32;
        let vy: i32 = (longitude * 10000.0) as i32;
        let vz: i32 = (meters * 100.0) as i32;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_GPS;
        self.buffer[{ self.index += 1; self.index }] = (vx >> 16) as u8;
        self.buffer[{ self.index += 1; self.index }] = (vx >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vx as u8;
        self.buffer[{ self.index += 1; self.index }] = (vy >> 16) as u8;
        self.buffer[{ self.index += 1; self.index }] = (vy >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vy as u8;
        self.buffer[{ self.index += 1; self.index }] = (vz >> 16) as u8;
        self.buffer[{ self.index += 1; self.index }] = (vz >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = vz as u8;
        self.index += 1;

        Ok(())
    }
}
