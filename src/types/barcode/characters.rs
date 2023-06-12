use crate::{std::fmt, tuple_struct, Error, Result};

pub const BARCODE_MIN_CHARS: u8 = 6;
pub const BARCODE_MAX_CHARS: u8 = 24;

tuple_struct!(
    BarcodeCharacters,
    u8,
    "Number of characters the barcode reader supports."
);

impl BarcodeCharacters {
    pub fn is_valid(&self) -> Result<()> {
        if (BARCODE_MIN_CHARS..=BARCODE_MAX_CHARS).contains(&self.0) {
            Ok(())
        } else {
            Err(Error::InvalidBarcodeCharacters(self.0))
        }
    }
}

impl fmt::Display for BarcodeCharacters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_inner())
    }
}
