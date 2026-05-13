/// System oscillator setup and control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Oscillator {
    /// Normal operation mode.
    On = 0b0000_0001,
    /// Standby mode.
    ///
    /// *This is the Power-on Reset default.*
    #[default]
    Off = 0b0000_0000,
}

impl Oscillator {
    pub(crate) fn as_command(&self) -> u8 {
        (0b0010 << 4) | *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            Oscillator::Off,
            Oscillator::default(),
            "Oscillator default is OFF"
        );
    }
}
