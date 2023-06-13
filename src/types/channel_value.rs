use crate::{make_list, std::fmt, tuple_struct_ser};

tuple_struct_ser!(ChannelValue, u32, "Channel denomination value.");

impl From<&[u8]> for ChannelValue {
    fn from(val: &[u8]) -> Self {
        let value = match val.len() {
            0 => 0,
            1 => u32::from_le_bytes([val[0], 0, 0, 0]),
            2 => u32::from_le_bytes([val[0], val[1], 0, 0]),
            3 => u32::from_le_bytes([val[0], val[1], val[2], 0]),
            _ => u32::from_le_bytes([val[0], val[1], val[2], val[3]]),
        };

        Self(value)
    }
}

impl<const N: usize> From<[u8; N]> for ChannelValue {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for ChannelValue {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for ChannelValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

make_list!(
    ChannelValueList,
    ChannelValue,
    "A list container for [ChannelValue]s."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_value() {
        let ch_val_5 = [0x05, 0x00, 0x00, 0x00];
        let ch_val_10 = [0x0a, 0x00, 0x00, 0x00];
        let ch_val_20 = [0x14, 0x00, 0x00, 0x00];
        let ch_val_50 = [0x32, 0x00, 0x00, 0x00];

        assert_eq!(ChannelValue::from(ch_val_5).as_inner(), 5);
        assert_eq!(ChannelValue::from(ch_val_10).as_inner(), 10);
        assert_eq!(ChannelValue::from(ch_val_20).as_inner(), 20);
        assert_eq!(ChannelValue::from(ch_val_50).as_inner(), 50);
    }
}
