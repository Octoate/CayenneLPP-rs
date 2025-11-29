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
    Power(u16),

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