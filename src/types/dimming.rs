/// Display dimming.
///
/// The whole display is dimmed via PWM @ (N + 1)/16 duty cycle; individual LEDs cannot be dimmed independently.
///
/// The value has to be in the inclusive range 0..15.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dimming(u8);

impl Default for Dimming {
    fn default() -> Dimming {
        Self::BRIGHTNESS_MAX
    }
}

impl Dimming {
    /// Return a validated `Dimming` value from the given `u8`.
    ///
    /// *NOTE: The brightness values are 0-indexed, e.g. `0u8` is equivalent to `1/16`, and `15u8` is `16/16`.*
    ///
    /// # Panics
    ///
    /// The value is validated to be in the inclusive range [`BRIGHTNESS_MIN`] to [`BRIGHTNESS_MAX`]. If
    /// the given `u8` value is too large then this function panics.
    ///
    /// # Example
    ///
    /// ```
    /// use ht16k33::Dimming;
    /// # use ht16k33::ValidationError;
    /// # fn main() -> Result<(), ValidationError> {
    ///
    /// let brightness = Dimming::new(1);
    ///
    /// assert_eq!(1u8, brightness.raw());
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Error Example
    ///
    /// ```should_panic
    /// use ht16k33::Dimming;
    /// use ht16k33::ValidationError;
    /// # fn main() {
    ///
    /// // Greater than the `BRIGHTNESS_MAX` value of `15u8`.
    /// let value = 16u8;
    ///
    /// let brightness = Dimming::new(value);
    /// # }
    /// ```
    ///
    pub fn new(value: u8) -> Self {
        if value <= 15 {
            Self(value)
        } else {
            panic!("Dimming value has to be <=15, was '{value}'")
        }
    }

    /// Return the raw value of the brightness level
    pub fn raw(self) -> u8 {
        self.0
    }

    pub(crate) fn as_command(self) -> u8 {
        (0b1110 << 4) | self.0
    }

    /// Minimum brightness @ 1/16 PWM
    pub const BRIGHTNESS_MIN: Self = Dimming(0);
    /// Maximum brightness @ 16/16 PWM
    pub const BRIGHTNESS_MAX: Self = Dimming(15);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_min() {
        assert_eq!(
            Dimming::new(0),
            Dimming::BRIGHTNESS_MIN,
            "Dimming MIN brightness doesn't match 1/16 value"
        );
    }

    #[test]
    fn brightness_max() {
        assert_eq!(
            Dimming::new(15),
            Dimming::BRIGHTNESS_MAX,
            "Dimming MAX brightness doesn't match 16/16 value"
        );
    }

    #[test]
    fn default() {
        assert_eq!(
            Dimming::BRIGHTNESS_MAX,
            Dimming::default(),
            "Dimming default is MAX brightness"
        );
    }

    #[test]
    fn from_u8() {
        for value in 0u8..16 {
            let dimming = Dimming::new(value);
            assert_eq!(value, dimming.0);
        }
    }

    #[test]
    #[should_panic]
    fn from_u8_too_large() {
        Dimming::new(16u8);
    }
}
