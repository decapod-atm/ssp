use crate::std::{self, fmt};

use crate::{bool_enum, impl_default};

pub mod bitmask {
    pub const SEQUENCE_ID: u8 = 0x7f;
}

bool_enum!(
    SequenceFlag,
    r"
 Each time the master sends a new packet to a device it alternates the sequence flag. If a
 device receives a packet with the same sequence flag as the last one, it does not execute
 the command but simply repeats its last reply. In a reply packet the address and sequence
 flag match the command packet.
"
);

impl SequenceFlag {
    /// Creates a new [SequenceFlag].
    ///
    /// Defaults to unset.
    pub const fn new() -> Self {
        Self::Unset
    }
}

impl_default!(SequenceFlag);

/// A combination of two items of data: the sequence flag (MSB, bit7) and the address of the
/// device (bit 6 to bit 0, LSB).
///
/// For example a SMART Hopper by default has an address of 0x10 (16 decimal). When the
/// sync bit is equal to 1 the byte sent to the Hopper is 0x90. On the next command, the sync
/// bit is toggled, in this case 0, the byte sent would be 0x10.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SequenceId {
    flag: SequenceFlag,
    id: u8,
}

impl SequenceId {
    /// Creates a new [SequenceId].
    pub const fn new() -> Self {
        Self {
            flag: SequenceFlag::new(),
            id: 0,
        }
    }

    /// Converts a u8 into a [SequenceId].
    pub const fn from_u8(b: u8) -> Self {
        let flag = SequenceFlag::from_u8(b >> 7);
        let id = b & bitmask::SEQUENCE_ID;

        Self { flag, id }
    }

    /// Creates a [SequenceId] from a [SequenceFlag] and ID.
    pub const fn from_parts(flag: SequenceFlag, id: u8) -> Self {
        Self { flag, id }
    }

    /// Gets the [SequenceFlag].
    pub fn flag(&self) -> SequenceFlag {
        self.flag
    }

    /// Sets the [SequenceFlag].
    pub fn set_flag(&mut self, flag: SequenceFlag) {
        self.flag = flag;
    }

    /// Toggles the value of the [SequenceFlag].
    pub fn toggle_flag(&mut self) {
        self.flag = !self.flag;
    }

    /// Gets the sequence ID.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Sets the sequence ID.
    pub fn set_id(&mut self, id: u8) {
        self.id = id & bitmask::SEQUENCE_ID;
    }

    /// Increments the sequence ID.
    ///
    /// If the ID reaches the maximum (`0x7F`), incrementing resets the value to zero.
    ///
    /// This is the behavior described in the documentation.
    pub fn increment(&mut self) -> u8 {
        self.id = (self.id + 1) & bitmask::SEQUENCE_ID;

        self.id
    }
}

impl_default!(SequenceId);

impl std::ops::Not for SequenceId {
    type Output = SequenceId;

    fn not(self) -> Self::Output {
        SequenceId::from_parts(!self.flag(), self.id())
    }
}

impl From<u8> for SequenceId {
    fn from(b: u8) -> Self {
        Self::from_u8(b)
    }
}

impl From<SequenceId> for u8 {
    fn from(s: SequenceId) -> Self {
        (u8::from(s.flag) << 7) | (s.id & bitmask::SEQUENCE_ID)
    }
}

impl fmt::Display for SequenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "flag({}): 0x{:02x}", self.flag, self.id)
    }
}
