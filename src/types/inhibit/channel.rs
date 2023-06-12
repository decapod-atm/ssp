use crate::{bool_enum, make_list, std::fmt, Error, Result};

bitfield! {
    /// Channel enable status bitfield.
    ///
    /// Each bit represent a channel on the device. If a channel is enabled the bit is set,
    /// otherwise the channel is disabled.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct EnableBitfield(u8);
    /// The 0th offset in this [ChannelInhibit].
    pub index0, set_index0: 0;
    /// The 1st offset in this [ChannelInhibit].
    pub index1, set_index1: 1;
    /// The 2nd offset in this [ChannelInhibit].
    pub index2, set_index2: 2;
    /// The 3rd offset in this [ChannelInhibit].
    pub index3, set_index3: 3;
    /// The 4th offset in this [ChannelInhibit].
    pub index4, set_index4: 4;
    /// The 5th offset in this [ChannelInhibit].
    pub index5, set_index5: 5;
    /// The 6th offset in this [ChannelInhibit].
    pub index6, set_index6: 6;
    /// The 7th offset in this [ChannelInhibit].
    pub index7, set_index7: 7;
}

impl EnableBitfield {
    /// Gets the [EnableChannel] based on `index`.
    pub fn channel(&self, index: usize) -> EnableChannel {
        match index % 8 {
            0 => self.index0().into(),
            1 => self.index1().into(),
            2 => self.index2().into(),
            3 => self.index3().into(),
            4 => self.index4().into(),
            5 => self.index5().into(),
            6 => self.index6().into(),
            _ => self.index7().into(),
        }
    }

    /// Sets the [EnableChannel] based on `index`.
    pub fn set_channel(&mut self, index: usize, enable: EnableChannel) {
        match index % 8 {
            0 => self.set_index0(enable.into()),
            1 => self.set_index1(enable.into()),
            2 => self.set_index2(enable.into()),
            3 => self.set_index3(enable.into()),
            4 => self.set_index4(enable.into()),
            5 => self.set_index5(enable.into()),
            6 => self.set_index6(enable.into()),
            _ => self.set_index7(enable.into()),
        }
    }
}

impl From<u8> for EnableBitfield {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<&u8> for EnableBitfield {
    fn from(val: &u8) -> Self {
        Self::from(*val)
    }
}

impl From<EnableBitfield> for u8 {
    fn from(val: EnableBitfield) -> Self {
        val.0
    }
}

impl From<&EnableBitfield> for u8 {
    fn from(val: &EnableBitfield) -> Self {
        val.0
    }
}

impl fmt::Display for EnableBitfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index0 = self.channel(0);
        let index1 = self.channel(1);
        let index2 = self.channel(2);
        let index3 = self.channel(3);
        let index4 = self.channel(4);
        let index5 = self.channel(5);
        let index6 = self.channel(6);
        let index7 = self.channel(7);

        write!(f, "Index0({index0}) : Index1({index1}) : Index2({index2}) : Index3({index3}) : Index4({index4}) : Index5({index5}) : Index6({index6}) : Index7({index7})")
    }
}

make_list!(
    EnableBitfieldList,
    EnableBitfield,
    r"A list of bitfields for enabling device channels."
);

bool_enum!(
    EnableChannel,
    r"Status flags for whether a channel on the device is inhibited"
);

/// Represents the number of configurable channels on a device.
///
/// This enum is specifically used with the [SetInhibitsCommand](crate::SetInhibitsCommand).
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InhibitChannels {
    Eight = 0x01,
    Sixteen = 0x02,
    TwentyFour = 0x03,
    ThirtyTwo = 0x04,
    SixtyFour = 0x08,
    Reserved = 0xff,
}

impl InhibitChannels {
    /// Converts the [InhibitChannels] to its number representation instead of the byte
    /// representation used in [SetInhibitsCommand](crate::SetInhibitsCommand) messages.
    pub fn to_number(&self) -> u8 {
        u8::from(self).saturating_mul(8)
    }

    /// Returns `Ok(_)` if the number of inhibit channels is supported, `Err(_)` otherwise.
    pub fn is_valid(&self) -> Result<()> {
        if matches!(self, &Self::Reserved) {
            Err(Error::InvalidInhibitChannels)
        } else {
            Ok(())
        }
    }
}

impl From<u8> for InhibitChannels {
    fn from(val: u8) -> Self {
        (val as usize).into()
    }
}

impl From<usize> for InhibitChannels {
    fn from(val: usize) -> Self {
        match val {
            0x01 => Self::Eight,
            0x02 => Self::Sixteen,
            0x03 => Self::TwentyFour,
            0x04 => Self::ThirtyTwo,
            0x05..=0x08 => Self::SixtyFour,
            _ => Self::Reserved,
        }
    }
}

impl From<InhibitChannels> for u8 {
    fn from(val: InhibitChannels) -> Self {
        val as u8
    }
}

impl From<&InhibitChannels> for u8 {
    fn from(val: &InhibitChannels) -> Self {
        (*val).into()
    }
}

impl From<InhibitChannels> for usize {
    fn from(val: InhibitChannels) -> Self {
        val as usize
    }
}

impl From<&InhibitChannels> for usize {
    fn from(val: &InhibitChannels) -> Self {
        (*val).into()
    }
}
