use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::SYNC_RESPONSE, message::MessageOps, MessageType,
};

/// Sync - Response (0x11)
///
/// Represents a response to an [SyncCommand](crate::SyncCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SyncResponse {
    buf: [u8; SYNC_RESPONSE],
}

impl SyncResponse {
    /// Creates a new [SyncResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; SYNC_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SyncResponse);
impl_message_from_buf!(SyncResponse);
impl_message_ops!(SyncResponse, MessageType::Synchronisation);
impl_response_ops!(SyncResponse);
impl_response_display!(SyncResponse);
