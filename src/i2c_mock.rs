//! # i2c_mock
//!
//! A mock I2C library to support using the [HT16K33](../struct.HT16K33.html) driver on non-Linux systems that do
//! not have I2C support.
use embedded_hal::{
    self as hal,
    i2c::{self, Operation},
};
use log::debug;

use core::fmt;

use crate::{Dimming, Display, constants::ROWS_SIZE};

/// Mock error to satisfy the I2C trait.
#[derive(Debug)]
pub enum I2cMockError {}

#[cfg(feature = "std")]
impl std::error::Error for I2cMockError {}

impl i2c::Error for I2cMockError {
    fn kind(&self) -> i2c::ErrorKind {
        match *self {}
    }
}

impl fmt::Display for I2cMockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2c MockError")
    }
}

/// The mock I2C state.
///
/// # Example
///
/// ```
/// use ht16k33::i2c_mock::I2cMock;
/// # fn main() {
///
/// // Create an I2cMock.
/// let i2c_mock = I2cMock::new();
///
/// # }
/// ```
pub struct I2cMock {
    /// Display RAM state.
    pub data_values: [u8; ROWS_SIZE],
}

impl I2cMock {
    /// Create an I2cMock.
    pub fn new() -> Self {
        I2cMock {
            data_values: [0; ROWS_SIZE],
        }
    }
}

impl Default for I2cMock {
    fn default() -> Self {
        Self::new()
    }
}

impl hal::i2c::ErrorType for I2cMock {
    type Error = I2cMockError;
}

impl hal::i2c::I2c for I2cMock {
    fn transaction(
        &mut self,
        _address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        // If there are no operations, we suceed
        let Some((first, remaining)) = operations.split_first_mut() else {
            return Ok(());
        };

        let Operation::Write(to_write) = first else {
            // Theoretically one could also do a single write transaction
            // and then a single read transaction etc. But that is bad style
            // (i think) and this is only the Mock anyway
            todo!("A read always needs a write command beforehand");
        };

        // Empty operation => ignore it and treat the remaining ones as
        // the real ones
        let Some((command, to_write)) = to_write.split_first() else {
            return self.transaction(_address, remaining);
        };

        match *command >> 4 {
            0b0000 => {
                let start_address = *command & 0x0F;
                if !to_write.is_empty() {
                    copy_to_wrapping_with_offset(
                        to_write,
                        &mut self.data_values,
                        start_address.into(),
                    );
                }
                for operation in remaining {
                    // I don't think sending more than one command in a single transaction would
                    // be spec compliant (because the dataseheet says that after a read a NACK + Stop
                    // is required)
                    let Operation::Read(buffer) = operation else {
                        todo!("A single transaction can only contain one command");
                    };

                    copy_from_wrapping_with_offset(buffer, &self.data_values, start_address.into());
                }
            }
            0b0010 => {
                if command & 1 == 1 {
                    debug!("Normal operation mode activated");
                } else {
                    debug!("Standby mode activated");
                }
            }
            0b1000 => {
                let display_status = Display::from_raw(command & 0x0F);
                debug!("Setting display to {display_status:#?}");
            }
            0b1110 => {
                let dimming_level = Dimming::new(*command & 0x0F);
                debug!("Setting dimming level to {dimming_level:#?}");
            }
            unhandled => unimplemented!("command byte '{unhandled:04b}' not yet implemented"),
        }

        Ok(())
    }
}

fn copy_to_wrapping_with_offset(src: &[u8], dst: &mut [u8], offset: usize) {
    let available = &mut dst[offset..];

    if src.len() <= available.len() {
        available[..src.len()].copy_from_slice(src);
    } else {
        available.copy_from_slice(&src[..available.len()]);
        copy_to_wrapping_with_offset(&src[available.len()..], dst, 0);
    }
}

fn copy_from_wrapping_with_offset(dst: &mut [u8], src: &[u8], offset: usize) {
    let available = &src[offset..];
    if dst.len() <= available.len() {
        dst.copy_from_slice(&available[..dst.len()]);
    } else {
        dst[..available.len()].copy_from_slice(available);
        copy_from_wrapping_with_offset(&mut dst[available.len()..], src, 0);
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal::i2c::I2c as _;

    use super::*;

    const ADDRESS: u8 = 0;

    #[test]
    fn new() {
        let _i2c_mock = I2cMock::new();
    }

    #[test]
    fn write_no_offset() {
        let mut i2c_mock = I2cMock::new();

        let write_buffer = [0, 1u8, 1u8];
        i2c_mock.write(ADDRESS, &write_buffer).unwrap();

        for value in 0..i2c_mock.data_values.len() {
            match value {
                0 | 1 => assert_eq!(
                    i2c_mock.data_values[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
                _ => assert_eq!(
                    i2c_mock.data_values[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
            }
        }
    }

    #[test]
    fn write_with_offset() {
        let mut i2c_mock = I2cMock::new();

        let offset = 4u8;
        let write_buffer = [offset, 1u8, 1u8];
        i2c_mock.write(ADDRESS, &write_buffer).unwrap();

        for value in 0..i2c_mock.data_values.len() {
            match value {
                4 | 5 => assert_eq!(
                    i2c_mock.data_values[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
                _ => assert_eq!(
                    i2c_mock.data_values[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
            }
        }
    }

    #[test]
    fn write_with_wraparound() {
        let mut i2c_mock = I2cMock::new();

        // Match the data values size, +2 to wrap around, +1 for the data command.
        let mut write_buffer = [1u8; super::ROWS_SIZE + 3];
        write_buffer[0] = 0;

        // These values should wrap and end up at indexes 0 & 1.
        write_buffer[write_buffer.len() - 1] = 2;
        write_buffer[write_buffer.len() - 2] = 2;

        i2c_mock.write(ADDRESS, &write_buffer).unwrap();

        for value in 0..i2c_mock.data_values.len() {
            match value {
                0 | 1 => assert_eq!(
                    i2c_mock.data_values[value], 2,
                    "index [{}] should be 2, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
                _ => assert_eq!(
                    i2c_mock.data_values[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
            }
        }
    }

    #[test]
    fn write_with_wraparound_and_offset() {
        let mut i2c_mock = I2cMock::new();

        // Match the data values size, +2 to wrap around, +1 for the data command.
        let mut write_buffer = [1u8; super::ROWS_SIZE + 3];

        let offset = 4u8;
        write_buffer[0] = offset;

        // These values should wrap and end up at indexes 4 & 5.
        write_buffer[write_buffer.len() - 1] = 2;
        write_buffer[write_buffer.len() - 2] = 2;

        i2c_mock.write(ADDRESS, &write_buffer).unwrap();

        for value in 0..i2c_mock.data_values.len() {
            match value {
                4 | 5 => assert_eq!(
                    i2c_mock.data_values[value], 2,
                    "index [{}] should be 2, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
                _ => assert_eq!(
                    i2c_mock.data_values[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, i2c_mock.data_values[value]
                ),
            }
        }
    }

    #[test]
    fn write_read_no_offset() {
        let mut i2c_mock = I2cMock::new();

        i2c_mock.data_values[0] = 1;
        i2c_mock.data_values[1] = 1;

        let mut read_buffer = [0u8; super::ROWS_SIZE];
        i2c_mock
            .write_read(ADDRESS, &[0], &mut read_buffer)
            .unwrap();

        #[allow(clippy::needless_range_loop)]
        for value in 0..read_buffer.len() {
            match value {
                0 | 1 => assert_eq!(
                    read_buffer[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, read_buffer[value]
                ),
                _ => assert_eq!(
                    read_buffer[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, read_buffer[value]
                ),
            }
        }
    }

    #[test]
    fn write_read_offset() {
        let mut i2c_mock = I2cMock::new();

        i2c_mock.data_values[2] = 1;
        i2c_mock.data_values[3] = 1;

        let mut read_buffer = [0u8; 4];

        let offset = 2u8;
        i2c_mock
            .write_read(ADDRESS, &[offset], &mut read_buffer)
            .unwrap();

        #[allow(clippy::needless_range_loop)]
        for value in 0..read_buffer.len() {
            match value {
                0 | 1 => assert_eq!(
                    read_buffer[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, read_buffer[value]
                ),
                _ => assert_eq!(
                    read_buffer[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, read_buffer[value]
                ),
            }
        }
    }

    #[test]
    fn write_read_wraparound() {
        let mut i2c_mock = I2cMock::new();

        i2c_mock.data_values[2] = 1;
        i2c_mock.data_values[3] = 1;

        let mut read_buffer = [0u8; super::ROWS_SIZE + 4];

        i2c_mock
            .write_read(ADDRESS, &[0], &mut read_buffer)
            .unwrap();

        #[allow(clippy::needless_range_loop)]
        for value in 0..read_buffer.len() {
            match value {
                2 | 3 | 18 | 19 => assert_eq!(
                    read_buffer[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, read_buffer[value]
                ),
                _ => assert_eq!(
                    read_buffer[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, read_buffer[value]
                ),
            }
        }
    }

    #[test]
    fn write_read_wraparound_and_offset() {
        let mut i2c_mock = I2cMock::new();

        i2c_mock.data_values[0] = 1;
        i2c_mock.data_values[1] = 1;

        let mut read_buffer = [0u8; super::ROWS_SIZE];

        let offset = 4u8;
        i2c_mock
            .write_read(ADDRESS, &[offset], &mut read_buffer)
            .unwrap();

        #[allow(clippy::needless_range_loop)]
        for value in 0..read_buffer.len() {
            match value {
                // The indexes will be 12/13 b/c the data values are at 1/2, but the read is offset
                // by 4, so the read buffer will wraparound to load those values.
                12 | 13 => assert_eq!(
                    read_buffer[value], 1,
                    "index [{}] should be 1, found [{}]",
                    value, read_buffer[value]
                ),
                _ => assert_eq!(
                    read_buffer[value], 0,
                    "index [{}] should be 0, found [{}]",
                    value, read_buffer[value]
                ),
            }
        }
    }
}
