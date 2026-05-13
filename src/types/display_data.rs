use bitflags::bitflags;

bitflags! {
    /// RAM data for LED display.
    ///
    /// The LED for the corresponding bitflag will be enabled if the flag is `1`.
    #[derive(Clone, Copy, Debug, Default,PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct DisplayData: u8 {
        /// No LEDs enabled.
        const ROW_NONE = 0b0000_0000;
        /// Led on column 0 enabled.
        const ROW_0 = 0b0000_0001;
        /// Led on column 1 enabled.
        const ROW_1 = 0b0000_0010;
        /// Led on column 2 enabled.
        const ROW_2 = 0b0000_0100;
        /// Led on column 3 enabled.
        const ROW_3 = 0b0000_1000;
        /// Led on column 4 enabled.
        const ROW_4 = 0b0001_0000;
        /// Led on column 5 enabled.
        const ROW_5 = 0b0010_0000;
        /// Led on column 6 enabled.
        const ROW_6 = 0b0100_0000;
        /// Led on column 7 enabled.
        const ROW_7 = 0b1000_0000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            DisplayData::ROW_NONE,
            DisplayData::default(),
            "DisplayData default ROW_NONE"
        );
    }

    #[test]
    fn all_on() {
        let data = DisplayData::ROW_0
            | DisplayData::ROW_1
            | DisplayData::ROW_2
            | DisplayData::ROW_3
            | DisplayData::ROW_4
            | DisplayData::ROW_5
            | DisplayData::ROW_6
            | DisplayData::ROW_7;

        assert_eq!(data, DisplayData::all(), "DisplayData is all enabled");
    }
}
