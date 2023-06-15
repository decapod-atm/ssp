use crate::std::{self, fmt};

use crate::{make_list, tuple_struct_ser};

tuple_struct_ser!(
    CountryCode,
    u32,
    "ASCII-encoded country code for the device."
);

impl From<&[u8]> for CountryCode {
    fn from(val: &[u8]) -> Self {
        let code = match val.len() {
            0 => 0,
            1 => u32::from_be_bytes([0, 0, 0, val[0]]),
            2 => u32::from_be_bytes([0, 0, val[0], val[1]]),
            _ => u32::from_be_bytes([0, val[0], val[1], val[2]]),
        };

        Self(code)
    }
}

impl<const N: usize> From<[u8; N]> for CountryCode {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for CountryCode {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_bytes = self.as_inner().to_be_bytes();

        let code = std::str::from_utf8(code_bytes[1..].as_ref()).unwrap_or("XXX");
        let code = code.trim_start_matches(['0', '\0']);

        write!(f, "{code}")
    }
}

make_list!(
    CountryCodeList,
    CountryCode,
    "A list container for [CountryCode]s."
);

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use alloc::format;

    use super::*;

    #[test]
    fn test_country_code() {
        let country_code_bytes = [0x45, 0x55, 0x52];
        let exp_country_code = CountryCode::from(b"EUR");
        let exp_country_code_str = "EUR";

        let country_code = CountryCode::from(country_code_bytes);

        assert_eq!(country_code, exp_country_code);
        assert_eq!(
            country_code.as_inner().to_be_bytes()[1..].as_ref(),
            b"EUR".as_ref()
        );
        assert_eq!(format!("{country_code}").as_str(), exp_country_code_str);
    }
}
