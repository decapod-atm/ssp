use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, IntermediateKey, MessageOps, MessageType,
};

pub mod index {
    pub const INTER_KEY: usize = 4;
    pub const INTER_KEY_END: usize = 12;
}

/// RequestKeyExchange - Command (0x4C)
///
/// The eight data bytes are a 64 bit number representing the Host intermediate key. If the
/// Generator and Modulus have been set the device will calculate the reply with the generic
/// response and eight data bytes representing the device intermediate key. The host and device
/// will then calculate the key.
///
/// If Generator and Modulus are not set then the device will reply FAIL.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RequestKeyExchangeCommand {
    buf: [u8; len::REQUEST_KEY_EXCHANGE_COMMAND],
}

impl RequestKeyExchangeCommand {
    /// Creates a new [RequestKeyExchangeCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::REQUEST_KEY_EXCHANGE_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::RequestKeyExchange);

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
    pub fn set_intermediate_key(&mut self, inter: &IntermediateKey) {
        let key_bytes = inter.as_inner().to_le_bytes();
        self.buf[index::INTER_KEY..index::INTER_KEY_END].copy_from_slice(key_bytes.as_ref());
    }
}

impl_default!(RequestKeyExchangeCommand);
impl_command_display!(RequestKeyExchangeCommand);
impl_message_from_buf!(RequestKeyExchangeCommand);
impl_message_ops!(RequestKeyExchangeCommand);
impl_command_ops!(RequestKeyExchangeCommand);
