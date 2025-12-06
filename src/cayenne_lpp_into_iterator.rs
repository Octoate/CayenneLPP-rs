use crate::CayenneLPP;
use crate::cayenne_lpp_scalar::{CayenneLPPScalar, CayenneLPPValue};
use crate::constants::*;
use crate::error::Error;

/// Iterator over the CayenneLPP Scalars parsed from a data structure
/// This version will return Option<Result<CayenneLPPScalar>>.  Some(Err(...))
/// will be returned when there was an error unpacking a CayenneLPP scalar.
/// the most likely causes of this error would be unhandled type codes,
/// out-of-bounds results, and stray bytes at the end of the bytestream.
/// Any of these could indicate corrupt data or, perhaps, that the provided
/// byte stream isn't actually in CayenneLPP format.
pub struct CayenneLPPIntoFailableIterator<'a> {
    /// The CayenneLPP instance that this iterator is over
    pub(crate) lpp: CayenneLPP<'a>,

    /// The current index into the CayenneLPP data structure.
    pub(crate) index: usize
}

impl<'a> CayenneLPPIntoFailableIterator<'a> {
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

impl<'a> Iterator for CayenneLPPIntoFailableIterator<'a> {
    type Item = Result<CayenneLPPScalar, Error>;

    fn next(&mut self) -> Option<Result<CayenneLPPScalar, Error>> {
        let buffer = &self.lpp.buffer;

        // Identify the case where we've gotten to the end of the
        // buffer cleanly, and we're done processing bytes.
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
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::DigitalInput(self.get_u8()) }
                ))
            },

            LPP_DIGITAL_OUTPUT => {
                let remaining_length = LPP_DIGITAL_OUTPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::DigitalOutput(self.get_u8()) }
                ))
            },

            LPP_ANALOG_INPUT => {
                let remaining_length = LPP_ANALOG_INPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let value = self.get_i16() as f32 / 100.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::AnalogInput(value)
                }))
            },

            LPP_ANALOG_OUTPUT => {
                let remaining_length = LPP_ANALOG_OUTPUT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let value = self.get_i16() as f32 / 100.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::AnalogOutput(value)
                }))
            },

            LPP_GENERIC_SENSOR => {
                let remaining_length = LPP_GENERIC_SENSOR_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::GenericSensor(self.get_u32())
                }))
            },

            LPP_LUMINOSITY => {
                let remaining_length = LPP_LUMINOSITY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Luminosity(self.get_u16())
                }))
            },

            LPP_PRESENCE => {
                let remaining_length = LPP_PERCENTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Presence(self.get_u8()) }
                ))
            },

            LPP_TEMPERATURE => {
                let remaining_length = LPP_TEMPERATURE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let value = self.get_i16() as f32 / 10.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Temperature(value)
                }))

            },

            LPP_RELATIVE_HUMIDITY => {
                let remaining_length = LPP_RELATIVE_HUMIDITY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let value = self.get_u8() as f32 / 2.0;

                // Relative humidity cannot be more than 100%, but
                // I don't neceesssarly cause an error if a sensor
                // is a little bit off, so we're not going to bounds
                // check here.  The natural value of the type bounds
                // it between 0 - 128%.

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::RelativeHumidity(value)
                }))
            },

            LPP_ACCELEROMETER => {
                let remaining_length = LPP_ACCELEROMETER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let vx = self.get_i16() as f32 / 1000.0;
                let vy = self.get_i16() as f32 / 1000.0;
                let vz = self.get_i16() as f32 / 1000.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Accelerometer(
                        vx, vy, vz
                    )
                }))
            },

            LPP_BAROMETRIC_PRESSURE => {
                let remaining_length = LPP_BAROMETRIC_PRESSURE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let pressure = self.get_u16() as f32 / 10.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::BarometricPressure(pressure)
                }))
            },

            LPP_VOLTAGE => {
                let remaining_length = LPP_VOLTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let voltage = self.get_u16() as f32 / 100.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Voltage(voltage)
                }))
            },

            LPP_CURRENT => {
                let remaining_length = LPP_CURRENT_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let amperage = self.get_u16() as f32 / 1000.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Current(amperage)
                }))
            },

            LPP_FREQUENCY => {
                let remaining_length = LPP_FREQUENCY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Frequency(self.get_u32())
                }))
            },

            LPP_PERCENTAGE => {
                let remaining_length = LPP_PERCENTAGE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Percentage(self.get_u8()) }
                ))
            },

            LPP_ALTITUDE => {
                let remaining_length = LPP_ALTITUDE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(
                    CayenneLPPScalar {
                        channel,
                        value: CayenneLPPValue::Altitude(self.get_i16())
                    }
                ))
            },

            LPP_POWER => {
                let remaining_length = LPP_POWER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Power(self.get_u16())
                }))
            },

            LPP_DISTANCE => {
                let remaining_length = LPP_DISTANCE_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Distance(self.get_u32())
                }))
            },

            LPP_ENERGY => {
                let remaining_length = LPP_ENERGY_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Energy(self.get_u32())
                }))
            },

            LPP_DIRECTION => {
                let remaining_length = LPP_DIRECTION_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                // I'm specifically not bounds checking direction because
                // I could see it being equally valid to use +/- to refer
                // to left or right of north, or to have directions larger
                // than 360 to indicate more than one turn.

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Direction(self.get_u16())
                }))
            },

            LPP_UNIXTIME => {
                let remaining_length = LPP_UNIXTIME_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::UnixTime(self.get_u32())
                }))
            },

            LPP_GYROMETER => {
                let remaining_length = LPP_GYROMETER_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let vx = self.get_u16() as f32 / 100.0;
                let vy = self.get_u16() as f32 / 100.0;
                let vz = self.get_u16() as f32 / 100.0;

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Gyrometer(
                        vx, vy, vz
                    )
                }))
            },

            LPP_GPS => {
                let remaining_length = LPP_GPS_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let lat = self.get_i24() as f32 / 10_000.0;
                let lon = self.get_i24() as f32 / 10_000.0;
                let alt = self.get_i24() as f32 / 100.0;

                // Do some basic sanity bounds checking.
                // The maximum latitude is +/- 90 degrees N/S
                if lat > 90.0 || lat < -90.0 {
                    return Some(Err(Error::OutOfRange));
                }

                // Same for longitude, but this time it's +/- 180.
                if lon > 180.0 || lon < -180.0 {
                    return Some(Err(Error::OutOfRange));
                }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::GPS(
                        lat,
                        lon,
                        alt)
                }))
            },

            LPP_SWITCH => {
                let remaining_length = LPP_SWITCH_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Switch(self.get_u8() != 0)
                }))
            },

            LPP_CONCENTRATION => {
                let remaining_length = LPP_CONCENTRATION_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Concentration(self.get_u16())
                }))

            },

            LPP_COLOR => {
                let remaining_length = LPP_COLOR_SIZE - 2;
                if buffer.len() < self.index + remaining_length { return Some(Err(Error::BufferUnderrun)) }

                let r = self.get_u8();
                let g = self.get_u8();
                let b = self.get_u8();

                Some(Ok(CayenneLPPScalar {
                    channel,
                    value: CayenneLPPValue::Color(r, g, b)
                }))
            },

            _ => Some(Err(Error::UnhandledType(type_code)))
        }
    }
}

pub struct CayenneLPPIterator<'a> {
    pub(crate) failable_iterator: CayenneLPPIntoFailableIterator<'a>
}

impl<'a> CayenneLPP<'a> {
    /// Create an infallable iterator.  This iterator flattens
    /// the result from a Option<Result<CayenneLPPScalar>> into
    /// just an Option.  This is useful if you want to just stop
    /// processing the byte stream when the first error is encountered.
    pub fn into_infailable_iter(self) -> CayenneLPPIterator<'a> {
        CayenneLPPIterator { failable_iterator: self.into_iter() }
    }
}

impl<'a> Iterator for CayenneLPPIterator<'a> {
    type Item = CayenneLPPScalar;

    fn next(&mut self) -> Option<CayenneLPPScalar> {
        match self.failable_iterator.next() {
            Some(Ok(s)) => Some(s),
            _ => None,
        }
    }     
}