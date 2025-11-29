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

/// Data type of switch value
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