use crate::constants::{COLUMN_SIZE, ROWS_SIZE};
use crate::errors::ValidationError;

/// Represents the LED location.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LedLocation {
    /// The Display RAM `row` address.
    pub row: u8,
    /// The Display RAM `column` data.
    pub column: u8,
}

impl LedLocation {
    /// Create an `LedLocation` with the given `row` and `column` values.
    ///
    /// # Errors
    ///
    /// The `row` and `column` values are validated to be within their respective [`ROWS_SIZE`] and
    /// [`COLUMNS_SIZE`] ranges of the device. If validation fails then [`ht16k33::ValidationError::ValueTooLarge`] is
    /// returned.
    ///
    /// [`ROWS_SIZE`]: constant.ROWS_SIZE.html
    /// [`COLUMNS_SIZE`]: constant.COLUMNS_SIZE.html
    /// [`ht16k33::ValidationError::ValueTooLarge`]: enum.ValidationError.html#variant.ValueTooLarge
    ///
    /// ```should_panic
    /// use ht16k33::LedLocation;
    /// use ht16k33::ValidationError;
    /// # use ht16k33::ROWS_SIZE;
    /// # fn main() {
    /// # let row = ROWS_SIZE as u8;
    /// # let column = 2u8;
    ///
    /// let location = match LedLocation::new(row, column) {
    ///     Ok(location) => location,
    ///     Err(ValidationError) => panic!(),
    /// };
    ///
    /// # }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new(row: u8, column: u8) -> Result<Self, ValidationError> {
        if row >= ROWS_SIZE as u8 {
            return Err(ValidationError::ValueTooLarge {
                name: "row",
                value: row,
                limit: ROWS_SIZE as u8,
                inclusive: false,
            });
        }

        if column >= COLUMN_SIZE as u8 {
            return Err(ValidationError::ValueTooLarge {
                name: "column",
                value: column,
                limit: COLUMN_SIZE as u8,
                inclusive: false,
            });
        }

        Ok(LedLocation { row, column })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn default() {
    //     let location = LedLocation::default();

    //     assert!(
    //         DisplayDataAddress::from_row(0) == location.row
    //             && DisplayData::COLUMN_NONE == location.column,
    //         "LedLocation default is (0, None)"
    //     );
    // }

    // #[test]
    // fn new() {
    //     let location = LedLocation::new(0, 0).unwrap();

    //     assert!(
    //         DisplayDataAddress::from_row(0) == location.row
    //             && DisplayData::COLUMN_0 == location.column,
    //         "LedLocation is (0, 0)"
    //     );

    //     let location = LedLocation::new(15, 7).unwrap();

    //     assert!(
    //         DisplayDataAddress::from_row(15) == location.row
    //             && DisplayData::COLUMN_7 == location.column,
    //         "LedLocation is (15, 7)"
    //     );
    // }

    #[test]
    #[should_panic]
    fn row_too_large() {
        let _ = LedLocation::new(16, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn column_too_large() {
        let _ = LedLocation::new(0, 8).unwrap();
    }

    // #[test]
    // fn row_as_index() {
    //     let location = LedLocation::new(2, 2).unwrap();
    //     assert_eq!(2usize, location.row_as_index());
    // }
}
