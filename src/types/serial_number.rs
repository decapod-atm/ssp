use crate::{std::fmt, tuple_struct};

tuple_struct!(SerialNumber, u32, "Serial number of a device.");

impl From<&[u8]> for SerialNumber {
    fn from(b: &[u8]) -> Self {
        match b.len() {
            0 => Self(0),
            1 => Self(u32::from_be_bytes([b[0], 0, 0, 0])),
            2 => Self(u32::from_be_bytes([b[0], b[1], 0, 0])),
            3 => Self(u32::from_be_bytes([b[0], b[1], b[2], 0])),
            _ => Self(u32::from_be_bytes([b[0], b[1], b[2], b[3]])),
        }
    }
}

impl<const N: usize> From<[u8; N]> for SerialNumber {
    fn from(b: [u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for SerialNumber {
    fn from(b: &[u8; N]) -> Self {
        b.as_ref().into()
    }
}

impl fmt::Display for SerialNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number() {
        let sn_bytes = [0x00, 0x29, 0xe9, 0x4b];
        let exp_sn = 2746699;

        assert_eq!(SerialNumber::from(sn_bytes).as_inner(), exp_sn);
    }
}
