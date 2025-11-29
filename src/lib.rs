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
//!
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

/// Data type of a generic sensor
pub const LPP_GENERIC_SENSOR: u8 =      100;     // 4 bytes, unsigned

/// Data type of a luminosity value
pub const LPP_LUMINOSITY: u8 =          101;     // 2 bytes, 1 lux unsigned

/// Data type of a presence sensor
pub const LPP_PRESENCE: u8 =            102;     // 1 byte, 1

/// Data type of a temperature value
pub const LPP_TEMPERATURE: u8 =         103;     // 2 bytes, 0.1°C signed

/// Data type of a relative humidity value
pub const LPP_RELATIVE_HUMIDITY: u8 =   104;     // 1 byte, 0.5% unsigned

/// Data type of accelerometer values
pub const LPP_ACCELEROMETER: u8 =       113;     // 2 bytes per axis, 0.001G

/// Data type of a barometric pressure value
pub const LPP_BAROMETRIC_PRESSURE: u8 = 115;     // 2 bytes 0.1 hPa Unsigned

/// Data type of a voltage value
pub const LPP_VOLTAGE: u8 =             116;     // 2 bytes 0.01V unsigned

/// Data type of a current value
pub const LPP_CURRENT: u8 =             117;     // 2 bytes 1mA unsigned

/// Data type of a frequency value
pub const LPP_FREQUENCY: u8 =           118;     // 4 bytes 1hz unsigned

/// Data type of a percentage
pub const LPP_PERCENTAGE: u8 =          120;     // 1 byte 1-100% unsigned

/// Data type of an altitude
pub const LPP_ALTITUDE: u8 =            121;     // 2 byte 1m signed

/// Data type of a concentration
pub const LPP_CONCENTRATION: u8 =       125;     // 2 bytes, 1ppm unsigned

/// Data type of a power value
pub const LPP_POWER: u8 =               128;     // 2 bytes 1W unsigned

/// Data type of a distance value
pub const LPP_DISTANCE: u8 =            130;     // 4 bytes, 0.001m unsigned

/// Data type of an energy value
pub const LPP_ENERGY: u8 =              131;     // 4 bytes, 1Wh, unsigned

/// Data type of a direction value
pub const LPP_DIRECTION: u8 =           132;     // 2 bytes, 1 deg, unsigned

/// Data type of a time (unix timestamp)
pub const LPP_UNIXTIME: u8 =            133;     // 4 bytes, unsigned

/// Data type of gyrometer values
pub const LPP_GYROMETER: u8 =           134;     // 2 bytes per axis, 0.01 °/s

/// Data type of a color value
pub const LPP_COLOR: u8 =               135;     // 1 byte per RGB color

/// Data type of GPS value
pub const LPP_GPS: u8 =                 136;     // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

/// Data type of a switch value
pub const LPP_SWITCH: u8 =              142;     // 1 byte 0/1

// Data ID + Data Type + Data Size
/// Size of a digital input packet including channel and data type
pub const LPP_DIGITAL_INPUT_SIZE: usize =       3;       // 1 byte

/// Size of a digital output packet including channel and data type
pub const LPP_DIGITAL_OUTPUT_SIZE: usize =      3;       // 1 byte

/// Size of an analog input packet including channel and data type
pub const LPP_ANALOG_INPUT_SIZE: usize =        4;       // 2 bytes, 0.01 signed

/// Size of an analog output packet including channel and data type
pub const LPP_ANALOG_OUTPUT_SIZE: usize =       4;       // 2 bytes, 0.01 signed

/// Size of a generic sensor packet including channel and data type
pub const LPP_GENERIC_SENSOR_SIZE: usize =      6;       // 4 bytes, unsigned

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
pub const LPP_BAROMETRIC_PRESSURE_SIZE: usize = 4;       // 2 bytes 0.1 hPa unsigned

/// Size of a voltage packet including channel and data type
pub const LPP_VOLTAGE_SIZE: usize =             4;       // 2 bytes 0.01V unsigned

/// Size of a current packet including channel and data type
pub const LPP_CURRENT_SIZE: usize =             4;      // 2 bytes 1mA unsigned

/// Size of a frequency packet including channel and data type
pub const LPP_FREQUENCY_SIZE: usize =           6;      // 4 bytes, 1hz unsigned

/// Size of an percentage packet including channel and data type
pub const LPP_PERCENTAGE_SIZE: usize =          3;      // 1 bytes, 0-100% unsigned

/// Size of an altitude packet including channel and data type
pub const LPP_ALTITUDE_SIZE: usize =            4;      // 2 bytes, 1M signed

/// Size of a concentration packet including channel and data type
pub const LPP_CONCENTRATION_SIZE: usize =       4;      // 2 bytes, 1ppm unsigned

/// Size of a power packet including channel and data type
pub const LPP_POWER_SIZE: usize =               4;      // 2 bytes, 1W unsigned

/// Size of a distance packet including channel and data type
pub const LPP_DISTANCE_SIZE: usize =            6;      // 4 bytes, 1mm unsigned

/// Size of an energy packet including channel and data type
pub const LPP_ENERGY_SIZE: usize =              6;      // 4 bytes, 1Wh unsigned

/// Size of a direction packet including channel and data type
pub const LPP_DIRECTION_SIZE: usize =           4;      // 2 bytes, 1deg, unsigned

/// Size of a unix time packet including channel and data type
pub const LPP_UNIXTIME_SIZE: usize =            6;      // 4 bytes, unsigned

/// Size of a gyrometer packet including channel and data type
pub const LPP_GYROMETER_SIZE: usize =           8;       // 2 bytes per axis, 0.01 °/s

/// Size of a color packet including channel and data type
pub const LPP_COLOR_SIZE: usize =               5;      // 1 bytes per color, RGB order

/// Size of a GPS packet including channel and data type
pub const LPP_GPS_SIZE: usize =                 11;      // 3 byte lon/lat 0.0001 °, 3 bytes alt 0.01 meter

/// Size of a switch packet including channel and data type
pub const LPP_SWITCH_SIZE: usize =              3;      // 1 byte 0/1

/// This struct contains the data of the added payload objects and an index that points to the next free
/// value in the array. All newly added values will increase the index. After adding all the values, the buffer
/// contains the payloads of the different data types and has a length of ''index''.
pub struct CayenneLPP<'a> {
    buffer: &'a mut [u8],
    index: usize
}

/// Defines all the errors that can occur in this crate.
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// The buffer is too small to add the value
    InsufficientMemory,
    /// The provided value is not representable by CayenneLPP
    OutOfRange,
}

impl<'a> CayenneLPP<'a> {

    /// Creates a new buffer for the Cayenne LPP. Since the library works without a memory allocator, it is necessary
    /// to provide it with an array that equals the length or is bigger then the data that shall be added to it.
    ///
    /// It is possible to use the size constants for the data types (e.g. LPP_TEMPERATURE_SIZE), to exactly calculate
    /// the size of the needed array.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        CayenneLPP {
            buffer,
            index: 0
        }
    }

    /// Resets the index that is pointing into the buffer, so it is possible to reuse the buffer and add new payloads
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
            CayenneLPPValue::Switch(s) => self.add_switch(channel, if s { 1 } else { 0 }),
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

/// Enumeration of the CayenneLPP value that are supported by this library
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CayenneLPPValue {
    /// Data type of a digital input
    DigitalInput(u8),

    /// Data type of a digital output
    DigitalOutput(u8),

    /// Data type of an analog input
    AnalogInput(f32),

    /// Data type of an analog output
    AnalogOutput(f32),

    /// Data type of a generic sensor
    GenericSensor(u32),

    /// Data type of a luminosity value
    Luminosity(u16),

    /// Data type of a presence sensor
    Presence(u8),

    /// Data type of a temperature value
    Temperature(f32),

    /// Data type of a relative humidity value
    RelativeHumidity(f32),
    
    /// Data type of accelerometer values
    Accelerometer(f32, f32, f32),

    /// Data type of a barometric pressure value
    BarometricPressure(f32),
   
    /// Data type of a voltage value
    Voltage(f32),

    /// Data type of a current value
    Current(f32),

    /// Data type of a frequency value
    Frequency(u32),

    /// Data type of a percentage
    Percentage(u8),

    /// Data type of an altitude
    Altitude(i16),

    /// Data type of a concentration
    Concentration(u16),

    /// Data type of a power value
    Power(i16),

    /// Data type of a distance value
    Distance(u32),

    /// Data type of an energy value
    Energy(u32),

    /// Data type of a direction value
    Direction(u16),
    /// Data type of a time (unix timestamp)
    UnixTime(u32),
    
    /// Data type of gyrometer values
    Gyrometer(f32, f32, f32),

    /// Data type of a color value
    Color(u8, u8, u8),

    /// Data type of GPS value
    GPS(f32, f32, f32),

    /// Data type of a switch value
    Switch(bool),
}

/// Single value parsed from a CayenneLPP data structure,
/// invluding the enumeration of its value and it's channel.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CayenneLPPScalar {
    /// The channel value parsed from a data structure
    pub channel: u8,

    /// Value of the calue parsed from a data structure
    pub value: CayenneLPPValue
}

/// Iterator over the CayenneLPP Scalars parsed from a datastructure
pub struct CayenneLPPIntoIterator<'a> {
    /// The CayenneLPP instance that this iterator is over
    lpp: CayenneLPP<'a>,

    /// The current index into the CayenneLPP datastructure.
    index: usize
}

impl<'a> CayenneLPPIntoIterator<'a> {
    // All of these functions are unsafe in the sense that they
    // rely on the size bounds being already checked.
    fn get_u32(&mut self) -> u32 {
        let byte_1 = self.lpp.buffer[self.index + 0] as u32;
        let byte_2 = self.lpp.buffer[self.index + 1] as u32;
        let byte_3 = self.lpp.buffer[self.index + 2] as u32;
        let byte_4 = self.lpp.buffer[self.index + 3] as u32;
        self.index += 4;

        let mut retval: u32 = 0;
        retval += byte_1 << 24;
        retval += byte_2 << 16;
        retval += byte_3 <<  8;
        retval += byte_4 <<  0;

        retval
    }

    /// Gets three bytes out of the byte array and coerces it into
    /// a 24-bit signed integer.  This is only used by the GPS packet.
    fn get_i24(&mut self) -> i32 {
        let byte_1 = self.lpp.buffer[self.index + 0] as i32;
        let byte_2 = self.lpp.buffer[self.index + 1] as i32;
        let byte_3 = self.lpp.buffer[self.index + 2] as i32;
        self.index += 3;

        let mut retval: i32 = 0;
        retval += byte_1 << 16;
        retval += byte_2 <<  8;
        retval += byte_3 <<  0;

        // Perform sign extension.  Because we're coming from a 24-bit
        // signed integer, and it's going into a 32-bit type, we need
        // to check whether the highest bit is a '1', and if so, ensure
        // the whole top byte is all ones.
        if 0x80 & byte_1 == 0x80 {
            retval += 0xFF000000u32 as i32;
        }

        retval
    }

    fn get_u16(&mut self) -> u16 {
        let byte_1 = self.lpp.buffer[self.index + 0] as u16;
        let byte_2 = self.lpp.buffer[self.index + 1] as u16;
        self.index += 2;

        let mut retval: u16 = 0;
        retval += byte_1 << 8;
        retval += byte_2 << 0;

        retval
    }

    fn get_i16(&mut self) -> i16 {
        self.get_u16() as i16
    }

    fn get_u8(&mut self) -> u8 {
        let retval = self.lpp.buffer[self.index];
        self.index += 1;
        retval
    }
}

impl<'a> Iterator for CayenneLPPIntoIterator<'a> {
    type Item = CayenneLPPScalar;

    fn next(&mut self) -> Option<CayenneLPPScalar> {
        let buffer = &self.lpp.buffer;
        
        // Identify the case where we've gotten to the end of the buffer
        if buffer.len() < self.index + 1 {
            return None
        }

        // Get the channel from the current index.  The index will
        // always be set to the first byte of the _next_ scalar.
        // when next is called.
        let channel = buffer[self.index];
        self.index += 1;
        let type_code = buffer[self.index];
        self.index += 1;

        match type_code {
            LPP_DIGITAL_INPUT => {
                let remaining_length = LPP_DIGITAL_INPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::DigitalInput(self.get_u8()) }
                )
            },

            LPP_DIGITAL_OUTPUT => {
                let remaining_length = LPP_DIGITAL_OUTPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::DigitalOutput(self.get_u8()) }
                )
            },

            LPP_ANALOG_INPUT => {
                let remaining_length = LPP_ANALOG_INPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let value = self.get_i16() as f32 / 100.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::AnalogInput(value) 
                })
            },

            LPP_ANALOG_OUTPUT => {
                let remaining_length = LPP_ANALOG_OUTPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let value = self.get_i16() as f32 / 100.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::AnalogOutput(value) 
                })
            },

            LPP_GENERIC_SENSOR => {
                let remaining_length = LPP_GENERIC_SENSOR_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::GenericSensor(self.get_u32())
                })
            },

            LPP_LUMINOSITY => {
                let remaining_length = LPP_LUMINOSITY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Luminosity(self.get_u16())
                })
            },

            LPP_PRESENCE => {
                let remaining_length = LPP_PERCRENTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Presence(self.get_u8()) }
                )
            },

            LPP_TEMPERATURE => {
                let remaining_length = LPP_TEMPERATURE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let value = self.get_i16() as f32 / 10.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Temperature(value) 
                })

            },

            LPP_RELATIVE_HUMIDITY => {
                let remaining_length = LPP_RELATIVE_HUMIDITY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let value = self.get_u8() as f32 / 2.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::RelativeHumidity(value)
                })
            },

            LPP_ACCELEROMETER => {
                let remaining_length = LPP_ACCELEROMETER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let vx = self.get_i16() as f32 / 1000.0;
                let vy = self.get_i16() as f32 / 1000.0;
                let vz = self.get_i16() as f32 / 1000.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Accelerometer(
                        vx, vy, vz
                    )
                })
            },

            LPP_BAROMETRIC_PRESSURE => {
                let remaining_length = LPP_BAROMETRIC_PRESSURE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let pressure = self.get_u16() as f32 / 10.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::BarometricPressure(pressure)
                })
            },

            LPP_VOLTAGE => {
                let remaining_length = LPP_VOLTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let voltage = self.get_u16() as f32 / 100.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Voltage(voltage)
                })
            },

            LPP_CURRENT => {
                let remaining_length = LPP_CURRENT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let amperage = self.get_u16() as f32 / 1000.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Current(amperage)
                })
            },

            LPP_FREQUENCY => {
                let remaining_length = LPP_FREQUENCY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Frequency(self.get_u32())
                })
            },

            LPP_PERCRENTAGE => {
                let remaining_length = LPP_PERCRENTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Percentage(self.get_u8()) }
                )
            },

            LPP_ALTITUDE => {
                let remaining_length = LPP_ALTITUDE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(
                    CayenneLPPScalar {
                        channel,
                        value: CayenneLPPValue::Altitude(self.get_i16())
                    }
                )
            },

            LPP_POWER => {
                let remaining_length = LPP_POWER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Power(self.get_i16())
                })
            },

            LPP_DISTANCE => {
                let remaining_length = LPP_DISTANCE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Distance(self.get_u32())
                })
            },

            LPP_ENERGY => {
                let remaining_length = LPP_ENERGY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Energy(self.get_u32())
                })
            },

            LPP_DIRECTION => {
                let remaining_length = LPP_DIRECTION_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Direction(self.get_u16())
                })
            },

            LPP_UNIXTIME => {
                let remaining_length = LPP_UNIXTIME_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::UnixTime(self.get_u32())
                })
            },

            LPP_GYROMETER => {
                let remaining_length = LPP_GYROMETER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let vx = self.get_u16() as f32 / 100.0;
                let vy = self.get_u16() as f32 / 100.0;
                let vz = self.get_u16() as f32 / 100.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Gyrometer(
                        vx, vy, vz
                    )
                })
            },

            LPP_GPS => {
                let remaining_length = LPP_GPS_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let lat = self.get_i24() as f32 / 10_000.0;
                let lon = self.get_i24() as f32 / 10_000.0;
                let alt = self.get_i24() as f32 / 100.0;

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::GPS(
                        lat,
                        lon,
                        alt)
                })
            },

            LPP_SWITCH => {
                let remaining_length = LPP_SWITCH_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Switch(self.get_u8() != 0)
                })
            },

            LPP_CONCENTRATION => {
                let remaining_length = LPP_CONCENTRATION_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Concentration(self.get_u16())
                })
                
            },

            LPP_COLOR => {
                let remaining_length = LPP_COLOR_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return None }

                let r = self.get_u8();
                let g = self.get_u8();
                let b = self.get_u8();

                Some(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Color(r, g, b)
                })
            },

            _ => None
        }
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