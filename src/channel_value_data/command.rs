use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::CHANNEL_VALUE_DATA_COMMAND, CommandOps, MessageOps, MessageType,
};

/// ChannelValueData - Command (0x0E)
///
/// Single byte command causes the validator to return the number of channels it is using
/// followed by the value of each channel.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChannelValueDataCommand {
    buf: [u8; CHANNEL_VALUE_DATA_COMMAND],
}

impl ChannelValueDataCommand {
    /// Creates a new [ChannelValueDataCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; CHANNEL_VALUE_DATA_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::ChannelValueData);

        msg
    }
}

impl_default!(ChannelValueDataCommand);
impl_command_display!(ChannelValueDataCommand);
impl_message_from_buf!(ChannelValueDataCommand);
impl_message_ops!(ChannelValueDataCommand);
impl_command_ops!(ChannelValueDataCommand);
