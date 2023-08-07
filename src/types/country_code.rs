use crate::make_list;

pub use currency_iso4217::Currency as CountryCode;

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
        let country_code_bytes: [u8; 3] = [0x45, 0x55, 0x52];
        let exp_country_code = CountryCode::from(b"EUR");
        let exp_country_code_str = r#""EUR""#;

        let country_code = CountryCode::from(country_code_bytes);

        assert_eq!(country_code, exp_country_code);
        assert_eq!(<&str>::from(country_code).as_bytes(), b"EUR".as_ref());
        assert_eq!(format!("{country_code}").as_str(), exp_country_code_str);
    }
}
