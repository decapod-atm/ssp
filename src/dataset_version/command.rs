use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// DatasetVersion - Command (0x21)
///
/// Single byte command returns a variable length ASCII-encoded array giving the installed dataset version
/// of the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DatasetVersionCommand {
    buf: [u8; len::DATASET_VERSION_COMMAND],
}

impl DatasetVersionCommand {
    /// Creates a new [DatasetVersionCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::DATASET_VERSION_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DatasetVersion);

        msg
    }
}

impl_default!(DatasetVersionCommand);
impl_command_display!(DatasetVersionCommand);
impl_message_from_buf!(DatasetVersionCommand);
impl_message_ops!(DatasetVersionCommand);
impl_command_ops!(DatasetVersionCommand);
