/// Enumeration of the CayenneLPP value that are supported by this library
#[derive(PartialEq, Clone, Copy)]
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

impl core::fmt::Debug for CayenneLPPValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DigitalInput(arg0) => {
                f.debug_tuple("DigitalInput").field(arg0).finish()
            },
            Self::DigitalOutput(arg0) => {
                f.debug_tuple("DigitalOutput").field(arg0).finish()
            },
            Self::AnalogInput(arg0) => {
                f.debug_tuple("AnalogInput").field(arg0).finish()
            },
            Self::AnalogOutput(arg0) => {
                f.debug_tuple("AnalogOutput").field(arg0).finish()
            },
            Self::GenericSensor(arg0) => {
                f.debug_tuple("GenericSensor").field(arg0).finish()
            },
            Self::Luminosity(arg0) => {
                f.debug_tuple("Luminosity (lux) ").field(arg0).finish()
            },
            Self::Presence(arg0) => {
                f.debug_tuple("Presence").field(arg0).finish()
            },
            Self::Temperature(arg0) => {
                f.debug_tuple("Degrees C").field(arg0).finish()
            },
            Self::RelativeHumidity(arg0) => {
                f.debug_tuple("RelativeHumidity").field(arg0).finish()
            },
            Self::Accelerometer(arg0, arg1, arg2) => {
                f.debug_struct("Accelerometer")
                    .field("X", arg0)
                    .field("Y", arg1)
                    .field("Z", arg2)
                    .finish()
            },
            Self::BarometricPressure(arg0) => {
                f.debug_tuple("BarometricPressure (hPa) ").field(arg0).finish()
            },
            Self::Voltage(arg0) => {
                f.debug_tuple("Voltage").field(arg0).finish()
            },
            Self::Current(arg0) => {
                f.debug_tuple("Current (A) ").field(arg0).finish()
            },
            Self::Frequency(arg0) => {
                f.debug_tuple("Frequency (hz) ").field(arg0).finish()
            },
            Self::Percentage(arg0) => {
                f.debug_tuple("Percentage").field(arg0).finish()
            },
            Self::Altitude(arg0) => {
                f.debug_tuple("Altitude (M) ").field(arg0).finish()
            },
            Self::Concentration(arg0) => {
                f.debug_tuple("Concentration (ppm) ").field(arg0).finish()
            },
            Self::Power(arg0) => {
                f.debug_tuple("Power (W) ").field(arg0).finish()
            },
            Self::Distance(arg0) => {
                f.debug_tuple("Distance (M) ").field(arg0).finish()
            },
            Self::Energy(arg0) => {
                f.debug_tuple("Energy (Wh) ").field(arg0).finish()
            },
            Self::Direction(arg0) => {
                f.debug_tuple("Direction (deg) ").field(arg0).finish()
            },
            Self::UnixTime(arg0) => {
                f.debug_tuple("UnixTime").field(arg0).finish()
            },
            Self::Gyrometer(arg0, arg1, arg2) => {
                f.debug_struct("Gyrometer")
                    .field("X", arg0)
                    .field("Y", arg1)
                    .field("Z", arg2)
                    .finish()
            },
            Self::Color(arg0, arg1, arg2) => {
                f.debug_struct("Color")
                    .field("R", arg0)
                    .field("G", arg1)
                    .field("B", arg2)
                    .finish()
            },
            Self::GPS(arg0, arg1, arg2) => {
                f.debug_struct("GPS location")
                    .field("Lat", arg0)
                    .field("Lon", arg1)
                    .field("Alt", arg2)
                    .finish()
            },
            Self::Switch(arg0) => {f.debug_tuple("Switch").field(arg0).finish()
            },
        }
    }
}

/// Single value parsed from a CayenneLPP data structure,
/// including the enumeration of its value and it's channel.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CayenneLPPScalar {
    /// The channel value parsed from a data structure
    pub channel: u8,

    /// Value of the calue parsed from a data structure
    pub value: CayenneLPPValue
}