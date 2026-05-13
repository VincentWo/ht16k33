/// The LED display state.
///
/// The LEDs can be all off (default), all on, or all blinking at 1/2Hz, 1Hz, or 2Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Display {
    /// Display off.
    /// *This is the Power-on Reset default.*
    #[default]
    Off = 0b0000,
    /// Display on; blinking off.
    On = 0b0001,
    /// Display on; blinking @ 2Hz.
    BlinkingTwoHz = 0b0011,
    /// Display on; blinking @ 1Hz.
    BlinkingOneHz = 0b0101,
    /// Display on; blinking @ 0.5Hz.
    BlinkingHalfHz = 0b0111,
}

impl Display {
    /// Get the command to set the corresponding state via I2C
    pub(crate) fn as_command(self) -> u8 {
        (0b1000 << 4) | self as u8
    }

    pub(crate) fn from_raw(raw: u8) -> Self {
        // Only three bits matter
        let raw = raw & 0b0111;
        if raw & 0b1 == 0 {
            Display::Off
        } else {
            match raw >> 1 {
                0b00 => Display::On,
                0b01 => Display::BlinkingTwoHz,
                0b10 => Display::BlinkingOneHz,
                0b11 => Display::BlinkingHalfHz,
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Display::Off, Display::default(), "Display default is OFF");
    }
}
