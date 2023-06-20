use crate::std::fmt;

use crate::tuple_struct_ser;

tuple_struct_ser!(ValueMultiplier, u32, "Multiplier for device note values.");

impl From<&[u8]> for ValueMultiplier {
    fn from(val: &[u8]) -> Self {
        let multi = match val.len() {
            0 => 0,
            1 => u32::from_be_bytes([0, 0, 0, val[0]]),
            2 => u32::from_be_bytes([0, 0, val[0], val[1]]),
            _ => u32::from_be_bytes([0, val[0], val[1], val[2]]),
        };

        Self(multi)
    }
}

impl<const N: usize> From<[u8; N]> for ValueMultiplier {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for ValueMultiplier {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for ValueMultiplier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:06x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use alloc::format;

    use super::*;

    #[test]
    fn test_value_multiplier() {
        let value_multi_bytes = [0x00, 0x00, 0x01];
        let exp_value_multi = 0x000001;
        let exp_value_multi_str = "0x000001";

        let value_multi = ValueMultiplier::from(value_multi_bytes);

        assert_eq!(value_multi.as_inner(), exp_value_multi);
        assert_eq!(format!("{value_multi}").as_str(), exp_value_multi_str);
    }
}
