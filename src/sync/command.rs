use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len::SYNC_COMMAND, CommandOps, MessageOps, MessageType,
};

/// Sync - Command (0x11)
///
/// This single byte command tells the unit that the next sequence ID will be 1.
///
/// This is always the first command sent to a unit, to prepare it to receive any further commands.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyncCommand {
    buf: [u8; SYNC_COMMAND],
}

impl SyncCommand {
    /// Creates a new [SyncCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; SYNC_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::Synchronisation);

        msg
    }
}

impl_default!(SyncCommand);
impl_command_display!(SyncCommand);
impl_message_from_buf!(SyncCommand);
impl_message_ops!(SyncCommand);
impl_command_ops!(SyncCommand);
