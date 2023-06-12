use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SetInhibits - Response (0x02)
///
/// Represents a response to an [SetInhibitsCommand](crate::SetInhibitsCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetInhibitsResponse {
    buf: [u8; len::SET_INHIBITS_RESPONSE],
}

impl SetInhibitsResponse {
    /// Creates a new [SetInhibitsResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_INHIBITS_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SetInhibitsResponse);
impl_message_from_buf!(SetInhibitsResponse);
impl_message_ops!(SetInhibitsResponse, MessageType::SetInhibits);
impl_response_ops!(SetInhibitsResponse);
impl_response_display!(SetInhibitsResponse);
