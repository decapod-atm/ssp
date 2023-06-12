use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_ops,
    len::SETUP_REQUEST_COMMAND, CommandOps, MessageOps, MessageType,
};

/// SetupRequest - Command (0x05)
///
/// Single byte command to request information about the current device setup configuration.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetupRequestCommand {
    buf: [u8; SETUP_REQUEST_COMMAND],
}

impl SetupRequestCommand {
    /// Creates a new [SetupRequestCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; SETUP_REQUEST_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetupRequest);

        msg
    }
}

impl_default!(SetupRequestCommand);
impl_command_display!(SetupRequestCommand);
impl_message_ops!(SetupRequestCommand);
impl_command_ops!(SetupRequestCommand);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_request_command() {
        let msg = SetupRequestCommand::new();

        assert_eq!(msg.message_type(), MessageType::SetupRequest);
    }
}
