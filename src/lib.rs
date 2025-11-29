#![no_std]
#![deny(missing_docs, unsafe_code, unstable_features, unused_import_braces, unused_qualifications, trivial_casts,
trivial_numeric_casts)]

//! This crate is a port of the [Cayenne LPP] (Low Power Payload) API. It provides an easy way to send data over LPWAN
//! networks such as LoRaWAN. Cayenne LPP is compliant with payload size restrictions, which can be lowered down to
//! 11 bytes and allows the device to send multiple sensor data at one time.
//!
//! Additionally, it is also possible to send different sensor data in different frames. To do this, the channel value
//! of the data can be used.
//!
//! The original C++ version of [Cayenne LPP] can be found [here].
//!
//! [Cayenne LPP]: https://docs.mydevices.com/docs/lorawan/cayenne-lpp
//! [here]: https://github.com/myDevicesIoT/CayenneLPP

pub use crate::constants::*;
pub use crate::cayenne_lpp_scalar::{CayenneLPPScalar, CayenneLPPValue};
use crate::error::Error;
use crate::cayenne_lpp_into_iterator::CayenneLPPIntoIterator;

pub(crate) mod constants;
pub(crate) mod cayenne_lpp_scalar;
mod cayenne_lpp_into_iterator;
mod error;
#[cfg(test)]
mod tests;

/// This struct contains the data of the added payload objects and an index that points to the next free
/// value in the array. All newly added values will increase the index. After adding all the values, the buffer
/// contains the payloads of the different data types and has a length of ''index''.
pub struct CayenneLPP<'a> {
    buffer: &'a mut [u8],
    index: usize
}

impl<'a> CayenneLPP<'a> {

    /// Creates a new buffer for the Cayenne LPP. Since the library works without a memory allocator, it is necessary
    /// to provide it with an array that equals the length or is bigger than the data that shall be added to it.
    ///
    /// It is possible to use the size constants for the data types (e.g., LPP_TEMPERATURE_SIZE) to exactly calculate
    /// the size of the necessary array.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        CayenneLPP {
            buffer,
            index: 0
        }
    }

    /// Resets the index pointing into the buffer, so it is possible to reuse the buffer and add new payloads
    /// to it.
    /// Remark: the buffer is not cleared by this operation.
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Returns the slice of the buffer that contains the payload of the added data types.
    pub fn payload_slice(&self) -> &[u8] {
        &self.buffer[0..self.index]
    }

    /// Adds a scalar value to the data structure.
    pub fn add_scalar(&mut self, scalar: &CayenneLPPScalar) -> Result<(), Error> {
        let channel = scalar.channel;
        match scalar.value {
            CayenneLPPValue::DigitalInput(s) => self.add_digital_input(channel, s),
            CayenneLPPValue::DigitalOutput(s) => self.add_digital_output(channel, s),
            CayenneLPPValue::AnalogInput(s) => self.add_analog_input(channel, s),
            CayenneLPPValue::AnalogOutput(s) => self.add_analog_output(channel, s),
            CayenneLPPValue::GenericSensor(s) => self.add_generic_sensor(channel, s),
            CayenneLPPValue::Luminosity(s) => self.add_luminosity(channel, s),
            CayenneLPPValue::Presence(s) => self.add_presence(channel, s),
            CayenneLPPValue::Temperature(s) => self.add_temperature(channel, s),
            CayenneLPPValue::RelativeHumidity(s) => self.add_relative_humidity(channel, s),
            CayenneLPPValue::Accelerometer(x, y, z) => self.add_accelerometer(channel, x, y, z),
            CayenneLPPValue::BarometricPressure(s) => self.add_barometric_pressure(channel, s),
            CayenneLPPValue::Voltage(s) => self.add_voltage(channel, s),
            CayenneLPPValue::Current(s) => self.add_current(channel, s),
            CayenneLPPValue::Frequency(s) => self.add_frequency(channel, s),
            CayenneLPPValue::Percentage(s) => self.add_percentage(channel, s),
            CayenneLPPValue::Altitude(s) => self.add_altitude(channel, s),
            CayenneLPPValue::Concentration(s) => self.add_concentration(channel, s),
            CayenneLPPValue::Power(s) => self.add_power(channel, s),
            CayenneLPPValue::Distance(s) => self.add_distance(channel, s),
            CayenneLPPValue::Energy(s) => self.add_energy(channel, s),
            CayenneLPPValue::Direction(s) => self.add_direction(channel, s),
            CayenneLPPValue::UnixTime(s) => self.add_unixtime(channel, s),
            CayenneLPPValue::Gyrometer(x, y, z) => self.add_gyrometer(channel, x, y, z),
            CayenneLPPValue::Color(r, g, b)        => self.add_color(channel, r, g, b),
            CayenneLPPValue::GPS(lat, lon, alt) => self.add_gps(channel, lat, lon, alt),
            CayenneLPPValue::Switch(s) => self.add_switch(channel, s),
        }
    }

    /// Adds the payload for a digital input to the Cayenne LPP data structure.
    pub fn add_digital_input(&mut self, channel: u8, value: u8) -> Result<(), Error> {
        if self.index + LPP_DIGITAL_INPUT_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_INPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a digital output to the Cayenne LPP data structure.
    pub fn add_digital_output(&mut self, channel: u8, value: u8) -> Result<(), Error> {
        if self.index + LPP_DIGITAL_OUTPUT_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIGITAL_OUTPUT;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for an analog input to the Cayenne LPP data structure.
    pub fn add_analog_input(&mut self, channel: u8, value: f32) -> Result<(), Error> {
        if self.index + LPP_ANALOG_INPUT_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for an analog output to the Cayenne LPP data structure.
    pub fn add_analog_output(&mut self, channel: u8, value: f32) -> Result<(), Error> {
        if self.index + LPP_ANALOG_OUTPUT_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for a generic sensor to the Cayenne LPP data structure. The units are not specified.
    pub fn add_generic_sensor(&mut self, channel: u8, value: u32) -> Result<(), Error> {
        if self.index + LPP_GENERIC_SENSOR_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let value_bytes = value.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_GENERIC_SENSOR;
        self.buffer[{ self.index += 1; self.index }] = value_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = value_bytes[1];
        self.buffer[{ self.index += 1; self.index }] = value_bytes[2];
        self.buffer[{ self.index += 1; self.index }] = value_bytes[3];
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for luminosity to the Cayenne LPP data structure. The value should be provided in lux.
    pub fn add_luminosity(&mut self, channel: u8, lux: u16) -> Result<(), Error> {
        if self.index + LPP_LUMINOSITY_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_LUMINOSITY;
        self.buffer[{ self.index += 1; self.index }] = (lux >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = lux as u8;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a presence sensor to the Cayenne LPP data structure.
    pub fn add_presence(&mut self, channel: u8, value: u8) -> Result<(), Error> {
        if self.index + LPP_PRESENCE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_PRESENCE;
        self.buffer[{ self.index += 1; self.index }] = value;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for temperature to the Cayenne LPP data structure.
    pub fn add_temperature(&mut self, channel: u8, celsius: f32) -> Result<(), Error> {
        if self.index + LPP_TEMPERATURE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for relative humidity to the Cayenne LPP data structure.
    pub fn add_relative_humidity(&mut self, channel: u8, relative_humidity: f32) -> Result<(), Error> {
        if self.index + LPP_RELATIVE_HUMIDITY_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let scaled_value = relative_humidity * 2.0;
        let fraction = scaled_value % 1.0;
        let whole_number = scaled_value - fraction;
        let rounded_value = match (whole_number, fraction) {
            // Greater than zero, Round toward zero
            (s, f) if s > 0.0 && f < 0.5 => {
                s
            },
            // Greater than zero, round away from zero
            (s, f) if s > 0.0 && f >= 0.5 => {
                s + 1.0
            }
            // Less than zero, round toward zero
            (s, f) if s < 0.0 && f > -0.5 => {
                s
            },
            // Less than zero, round away from zero
            (s, f) if s < 0.0 && f <= -0.5 => {
                s - 1.0
            },
            // Should never, happen, just clamp if it does...
            (s, _) => {
                s
            }
        };

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_RELATIVE_HUMIDITY;
        self.buffer[{ self.index += 1; self.index }] = rounded_value as u8;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload of an accelerometer to the Cayenne LPP data structure.
    pub fn add_accelerometer(&mut self, channel: u8, x: f32, y: f32, z: f32) -> Result<(), Error> {
        if self.index + LPP_ACCELEROMETER_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for barometric pressure to the Cayenne LPP data structure.
    pub fn add_barometric_pressure(&mut self, channel: u8, hpa: f32) -> Result<(), Error> {
        if self.index + LPP_BAROMETRIC_PRESSURE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let pressure = (hpa * 10.0) as u16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_BAROMETRIC_PRESSURE;
        self.buffer[{ self.index += 1; self.index }] = (pressure >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = pressure as u8;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a voltage to the Cayenne LPP data structure (in volts)
    pub fn add_voltage(&mut self, channel: u8, voltage: f32) -> Result<(), Error> {
        if self.index + LPP_VOLTAGE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        if voltage * 100.0 > u16::MAX as f32 {
            return Err(Error::OutOfRange);
        }
    
        let voltage: u16 = (voltage * 100.0) as u16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_VOLTAGE;
        self.buffer[{ self.index += 1; self.index }] = (voltage >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = voltage as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for a current to the Cayenne LPP data structure (in amps)
    pub fn add_current(&mut self, channel: u8, amperage: f32) -> Result<(), Error> {
        if self.index + LPP_CURRENT_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        if amperage * 1000.0 > u16::MAX as f32 {
            return Err(Error::OutOfRange);
        }
    
        let amperage: u16 = (amperage * 1000.0) as u16;

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_CURRENT;
        self.buffer[{ self.index += 1; self.index }] = (amperage >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = amperage as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for a frequency to the Cayenne LPP data structure. The units are in hertz
    pub fn add_frequency(&mut self, channel: u8, frequency: u32) -> Result<(), Error> {
        if self.index + LPP_FREQUENCY_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let frequency_bytes = frequency.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_FREQUENCY;
        self.buffer[{ self.index += 1; self.index }] = frequency_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = frequency_bytes[1];
        self.buffer[{ self.index += 1; self.index }] = frequency_bytes[2];
        self.buffer[{ self.index += 1; self.index }] = frequency_bytes[3];
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a percentage to the CayenneLPP data structure.  The units are single percent (0-100)%
    pub fn add_percentage(&mut self, channel: u8, percentage: u8) -> Result<(), Error> {
        if self.index + LPP_PERCENTAGE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_PERCENTAGE;
        self.buffer[{ self.index += 1; self.index }] = percentage;
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a altitude to the Cayenne LPP data structure (in meters)
    pub fn add_altitude(&mut self, channel: u8, altitude: i16) -> Result<(), Error> {
        if self.index + LPP_ALTITUDE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_ALTITUDE;
        self.buffer[{ self.index += 1; self.index }] = (altitude >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = altitude as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for a power to the Cayenne LPP data structure (in watts)
    pub fn add_power(&mut self, channel: u8, power: u16) -> Result<(), Error> {
        if self.index + LPP_POWER_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_POWER;
        self.buffer[{ self.index += 1; self.index }] = (power >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = power as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for a frequency to the Cayenne LPP data structure. The units are in millimeters
    pub fn add_distance(&mut self, channel: u8, distance: u32) -> Result<(), Error> {
        if self.index + LPP_DISTANCE_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let distance_bytes = distance.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DISTANCE;
        self.buffer[{ self.index += 1; self.index }] = distance_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = distance_bytes[1];
        self.buffer[{ self.index += 1; self.index }] = distance_bytes[2];
        self.buffer[{ self.index += 1; self.index }] = distance_bytes[3];
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a energy to the Cayenne LPP data structure. The units are in single Wh
    pub fn add_energy(&mut self, channel: u8, energy: u32) -> Result<(), Error> {
        if self.index + LPP_ENERGY_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let energy_bytes = energy.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_ENERGY;
        self.buffer[{ self.index += 1; self.index }] = energy_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = energy_bytes[1];
        self.buffer[{ self.index += 1; self.index }] = energy_bytes[2];
        self.buffer[{ self.index += 1; self.index }] = energy_bytes[3];
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a direction to the Cayenne LPP data structure (in degrees)
    pub fn add_direction(&mut self, channel: u8, direction: u16) -> Result<(), Error> {
        if self.index + LPP_DIRECTION_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_DIRECTION;
        self.buffer[{ self.index += 1; self.index }] = (direction >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = direction as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for a unixtime to the Cayenne LPP data structure.
    /// The units are in seconds, and it's relative to unix epoch
    pub fn add_unixtime(&mut self, channel: u8, unixtime: u32) -> Result<(), Error> {
        if self.index + LPP_UNIXTIME_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        let unixtime_bytes = unixtime.to_be_bytes();

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_UNIXTIME;
        self.buffer[{ self.index += 1; self.index }] = unixtime_bytes[0];
        self.buffer[{ self.index += 1; self.index }] = unixtime_bytes[1];
        self.buffer[{ self.index += 1; self.index }] = unixtime_bytes[2];
        self.buffer[{ self.index += 1; self.index }] = unixtime_bytes[3];
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a gyrometer to the Cayenne LPP data structure.
    pub fn add_gyrometer(&mut self, channel: u8, x: f32, y: f32, z: f32) -> Result<(), Error> {
        if self.index + LPP_GYROMETER_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for GPS to the Cayenne LPP data structure.
    pub fn add_gps(&mut self, channel: u8, latitude: f32, longitude: f32, meters: f32) -> Result<(), Error> {
        if self.index + LPP_GPS_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
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

    /// Adds the payload for switch to the Cayenne LPP data structure. It's a byte that's just 0/1
    pub fn add_switch(&mut self, channel: u8, value: bool) -> Result<(), Error> {
        if self.index + LPP_SWITCH_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_SWITCH;
        self.buffer[{ self.index += 1; self.index }] = if value { 1 } else { 0 };
        self.index += 1;

        Ok(())
    }

    /// Adds the payload for a concentration to the Cayenne LPP data structure (in ppm)
    pub fn add_concentration(&mut self, channel: u8, concentration: u16) -> Result<(), Error> {
        if self.index + LPP_CONCENTRATION_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_CONCENTRATION;
        self.buffer[{ self.index += 1; self.index }] = (concentration >> 8) as u8;
        self.buffer[{ self.index += 1; self.index }] = concentration as u8;
        self.index += 1;
        
        Ok(())
    }

    /// Adds the payload for color to the Cayenne LPP data structure. It's a byte per color channel
    pub fn add_color(&mut self, channel: u8, red: u8, green: u8, blue: u8) -> Result<(), Error> {
        if self.index + LPP_COLOR_SIZE > self.buffer.len() {
            return Err(Error::InsufficientMemory);
        }

        self.buffer[self.index] = channel;
        self.buffer[{ self.index += 1; self.index }] = LPP_COLOR;
        self.buffer[{ self.index += 1; self.index }] = red;
        self.buffer[{ self.index += 1; self.index }] = green;
        self.buffer[{ self.index += 1; self.index }] = blue;
        self.index += 1;

        Ok(())
    }
}

impl<'a> IntoIterator for CayenneLPP<'a> {
    type Item = CayenneLPPScalar;
    type IntoIter = CayenneLPPIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CayenneLPPIntoIterator {
            lpp: self,
            index: 0
        }
    }
}