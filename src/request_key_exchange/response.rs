use crate::{
    impl_default, impl_message_from_buf, impl_response_display, impl_response_ops,
    impl_var_message_ops, len, IntermediateKey, MessageOps, MessageType,
};

mod index {
    pub const INTER_KEY: usize = 4;
    pub const INTER_KEY_END: usize = 12;
}

/// RequestKeyExchange - Response (0x4B)
///
/// Represents a response to a [RequestKeyExchangeCommand](crate::RequestKeyExchangeCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RequestKeyExchangeResponse {
    buf: [u8; len::REQUEST_KEY_EXCHANGE_RESPONSE],
}

impl RequestKeyExchangeResponse {
    /// Creates a new [RequestKeyExchangeResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::REQUEST_KEY_EXCHANGE_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [IntermediateKey].
    pub fn intermediate_key(&self) -> IntermediateKey {
        let key_bytes: [u8; 8] = self.buf[index::INTER_KEY..index::INTER_KEY_END]
            .try_into()
            .unwrap();
        u64::from_le_bytes(key_bytes).into()
    }

    /// Sets the [IntermediateKey].
    pub fn set_intermediate_key(&mut self, key: IntermediateKey) {
        let key_bytes = key.as_inner().to_le_bytes();
        self.buf[index::INTER_KEY..index::INTER_KEY_END].copy_from_slice(key_bytes.as_ref());
    }
}

impl_default!(RequestKeyExchangeResponse);
impl_message_from_buf!(RequestKeyExchangeResponse);
impl_var_message_ops!(RequestKeyExchangeResponse, MessageType::RequestKeyExchange);
impl_response_ops!(RequestKeyExchangeResponse);
impl_response_display!(RequestKeyExchangeResponse);
