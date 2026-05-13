/// Display RAM data address.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisplayDataAddress {
    row: u8,
}

impl DisplayDataAddress {
    /// Create the address corresponding to the specified row.
    /// The row must be <= 15
    ///
    /// # Panics
    /// This function panics of `row` is larger then 15
    pub fn from_row(row: u8) -> Self {
        if row <= 15 {
            Self { row }
        } else {
            panic!("Invalid row number '{row}', only rows 0..=15 can be adressed.")
        }
    }
}
