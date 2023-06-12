use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf,
    impl_var_message_ops, len, std, CommandOps, EnableBitfield, EnableBitfieldList, EnableChannel,
    Error, InhibitChannels, MessageOps, MessageType, Result, Vec,
};

pub mod index {
    pub const INHIBIT: usize = 4;
}

/// SetInhibits - Command (0x02)
///
/// Sets the channel inhibit level for the device, each byte sent represents 8 bits (channels of
/// inhibit).
///
/// Nv200 has the option to send 2, 3, or 4 bytes to represent 16, 24, or 32 channels, the other
/// BNV devices have the option of sending 1 or 2 bytes for 8 or 16 channel operation.
///
/// Set the bit low to inhibit all note acceptance on that channel, high to allow note acceptance.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetInhibitsCommand {
    buf: [u8; len::SET_INHIBITS_COMMAND],
}

impl SetInhibitsCommand {
    /// Creates a new [SetInhibitsCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_INHIBITS_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetInhibits);

        msg
    }

    /// Gets the number of [InhibitChannels].
    pub fn num_channels(&self) -> InhibitChannels {
        (self.data_len() - 1).into()
    }

    /// Gets the [EnableChannel] setting.
    pub fn inhibit(&self, index: usize) -> Result<EnableChannel> {
        self.check_num_channels(index)?;

        let buf_index = index::INHIBIT + Self::bitfield_index(index);

        Ok(EnableBitfield::from(self.buf[buf_index]).channel(index))
    }

    /// Gets the [EnableBitfield] settings for all channels.
    pub fn inhibits(&self) -> EnableBitfieldList {
        self.buf[self.inhibit_start()..self.inhibit_end()]
            .iter()
            .map(|b| EnableBitfield::from(b))
            .collect::<Vec<EnableBitfield>>()
            .into()
    }

    /// Sets the enabled status of the given channel (by index).
    pub fn set_inhibit(&mut self, index: usize, enable: EnableChannel) -> Result<()> {
        self.check_num_channels(index)?;

        let buf_index = index::INHIBIT + Self::bitfield_index(index);

        let mut bitfield = EnableBitfield::from(self.buf[buf_index]);
        bitfield.set_channel(index, enable);

        self.buf[buf_index] = bitfield.into();

        Ok(())
    }

    /// Sets the entire [EnableBitfield] for a set of channels.
    pub fn set_inhibit_bitfield(&mut self, index: usize, enable: EnableBitfield) -> Result<()> {
        self.check_num_channels(index)?;

        let index = self.inhibit_start() + Self::bitfield_index(index);

        self.buf[index] = enable.into();

        Ok(())
    }

    /// Sets the number of inhibit channels, and which channels are enabled.
    ///
    /// The length of the [EnableBitfieldList] must be a valid variant of [InhibitChannels].
    pub fn set_inhibits(&mut self, enable_list: EnableBitfieldList) -> Result<()> {
        let len = enable_list.len();
        let chan_len = InhibitChannels::from(len);

        chan_len.is_valid()?;

        self.set_data_len((chan_len as u8) + 1);

        let start = self.inhibit_start();
        let list_end = start + len;
        let end = std::cmp::min(list_end, self.inhibit_end());

        for (set_enable, enable) in self.buf[start..end].iter_mut().zip(enable_list.iter()) {
            *set_enable = enable.into();
        }

        Ok(())
    }

    fn bitfield_index(index: usize) -> usize {
        index / 8
    }

    fn check_num_channels(&self, index: usize) -> Result<()> {
        let num_channels = self.num_channels().to_number() as usize;
        if index >= num_channels {
            Err(Error::InvalidLength((index, num_channels)))
        } else {
            Ok(())
        }
    }

    fn inhibit_start(&self) -> usize {
        index::INHIBIT
    }

    fn inhibit_end(&self) -> usize {
        self.inhibit_start() + usize::from(self.num_channels())
    }
}

impl_default!(SetInhibitsCommand);
impl_command_display!(SetInhibitsCommand);
impl_message_from_buf!(SetInhibitsCommand);
impl_var_message_ops!(SetInhibitsCommand);
impl_command_ops!(SetInhibitsCommand);
