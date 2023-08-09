use crate::{std::fmt, tuple_struct};

tuple_struct!(
    SequenceCount,
    u32,
    r"
 A 4 byte unsigned integer representing the sequence count of encrypted packages.

 Encrypted packets are sequenced using this count; this is reset to 0 after a power cycle,
 and each time the encryption keys are successfully negotiated.

 The count is incremented by the host and device each time they successfully encrypt and transmit a packet, and each time a received packet is successfully decrypted.

 After a packet is successfully decrypted the COUNT in the packet should be compared with the internal COUNT, if they do not match then the packet is discarded.
"
);

impl From<&[u8]> for SequenceCount {
    fn from(val: &[u8]) -> Self {
        match val.len() {
            0 => Self::new(),
            1 => u32::from_le_bytes([val[0], 0, 0, 0]).into(),
            2 => u32::from_le_bytes([val[0], val[1], 0, 0]).into(),
            3 => u32::from_le_bytes([val[0], val[1], val[2], 0]).into(),
            _ => u32::from_le_bytes([val[0], val[1], val[2], val[3]]).into(),
        }
    }
}

impl<const N: usize> From<[u8; N]> for SequenceCount {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[u8; N]> for SequenceCount {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for SequenceCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_inner())
    }
}
