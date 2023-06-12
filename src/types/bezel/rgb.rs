use crate::{std::fmt, tuple_struct};

tuple_struct!(Red, u8, "Configures the RED setting of the bezel color.");
tuple_struct!(
    Green,
    u8,
    "Configures the GREEN setting of the bezel color."
);
tuple_struct!(Blue, u8, "Configures the BLUE setting of the bezel color.");

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.as_inner())
    }
}

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.as_inner())
    }
}

impl fmt::Display for Blue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.as_inner())
    }
}

/// Represents the RED-GREEN-BLUE settings of the bezel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB {
    red: Red,
    green: Green,
    blue: Blue,
}

impl RGB {
    /// Creates a new [RGB].
    pub const fn new() -> Self {
        Self {
            red: Red(0x00),
            green: Green(0x00),
            blue: Blue(0x00),
        }
    }

    /// Gets the [Red] setting.
    pub fn red(&self) -> Red {
        self.red
    }

    /// Sets the [Red] setting.
    pub fn set_red<R: Into<Red>>(&mut self, red: R) {
        self.red = red.into();
    }

    /// Gets the [Green] setting.
    pub fn green(&self) -> Green {
        self.green
    }

    /// Sets the [Green] setting.
    pub fn set_green<G: Into<Green>>(&mut self, green: G) {
        self.green = green.into();
    }

    /// Gets the [Blue] setting.
    pub fn blue(&self) -> Blue {
        self.blue
    }

    /// Sets the [Blue] setting.
    pub fn set_blue<B: Into<Blue>>(&mut self, blue: B) {
        self.blue = blue.into();
    }

    /// Converts the [RGB] settings into a byte array.
    pub fn as_bytes(&self) -> [u8; 3] {
        [self.red.into(), self.green.into(), self.blue.into()]
    }
}

impl From<&[u8]> for RGB {
    fn from(val: &[u8]) -> Self {
        match val.len() {
            0 => Self::new(),
            1 => Self {
                red: val[0].into(),
                green: Green(0),
                blue: Blue(0),
            },
            2 => Self {
                red: val[0].into(),
                green: val[1].into(),
                blue: Blue(0),
            },
            _ => Self {
                red: val[0].into(),
                green: val[1].into(),
                blue: val[2].into(),
            },
        }
    }
}

impl<const N: usize> From<[u8; N]> for RGB {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for RGB {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl From<RGB> for [u8; 3] {
    fn from(val: RGB) -> Self {
        val.as_bytes()
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let red = self.red();
        let green = self.green();
        let blue = self.blue();

        write!(f, "#{red}{green}{blue}")
    }
}
