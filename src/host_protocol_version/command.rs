use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::HOST_PROTOCOL_VERSION_COMMAND, CommandOps, MessageOps, MessageType, ProtocolVersion,
};

pub mod index {
    pub const PROTOCOL_VERSION: usize = 4;
}

/// HostProtocolVersion - Command (0x0A)
///
/// Single byte command enables the unit. It will now respond to and execute commands.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HostProtocolVersionCommand {
    buf: [u8; HOST_PROTOCOL_VERSION_COMMAND],
}

impl HostProtocolVersionCommand {
    /// Creates a new [HostProtocolVersionCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; HOST_PROTOCOL_VERSION_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::HostProtocolVersion);

        msg
    }

    /// Gets the [ProtocolVersion].
    pub fn version(&self) -> ProtocolVersion {
        self.buf[index::PROTOCOL_VERSION].into()
    }

    /// Sets the [ProtocolVersion].
    pub fn set_version(&mut self, version: ProtocolVersion) {
        self.buf[index::PROTOCOL_VERSION] = version.into();
    }
}

impl_default!(HostProtocolVersionCommand);
impl_command_display!(HostProtocolVersionCommand);
impl_message_from_buf!(HostProtocolVersionCommand);
impl_message_ops!(HostProtocolVersionCommand);
impl_command_ops!(HostProtocolVersionCommand);
