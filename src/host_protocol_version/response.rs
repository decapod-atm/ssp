use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::HOST_PROTOCOL_VERSION_RESPONSE, message::MessageOps, MessageType,
};

/// HostProtocolVersion - Response (0x06)
///
/// Represents a response to an [HostProtocolVersionCommand](crate::HostProtocolVersionCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HostProtocolVersionResponse {
    buf: [u8; HOST_PROTOCOL_VERSION_RESPONSE],
}

impl HostProtocolVersionResponse {
    /// Creates a new [HostProtocolVersionResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; HOST_PROTOCOL_VERSION_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(HostProtocolVersionResponse);
impl_message_from_buf!(HostProtocolVersionResponse);
impl_message_ops!(
    HostProtocolVersionResponse,
    MessageType::HostProtocolVersion
);
impl_response_ops!(HostProtocolVersionResponse);
impl_response_display!(HostProtocolVersionResponse);
