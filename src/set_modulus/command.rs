use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType, ModulusKey,
};

mod index {
    pub const MODULUS: usize = 4;
    pub const MODULUS_END: usize = 12;
}

/// SetModulus - Command (0x4B)
///
/// Eight data bytes are sent. This is a 64 bit number representing the Modulus, and must be a
/// prime number.
///
/// The device will reply with OK or PARAMETER_OUT_OF_RANGE if the number is not prime.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetModulusCommand {
    buf: [u8; len::SET_MODULUS_COMMAND],
}

impl SetModulusCommand {
    /// Creates a new [SetModulusCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_MODULUS_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetModulus);

        msg
    }

    /// Gets the prime [ModulusKey].
    pub fn modulus(&self) -> ModulusKey {
        let key_bytes: [u8; 8] = self.buf[index::MODULUS..index::MODULUS_END]
            .try_into()
            .unwrap();
        u64::from_le_bytes(key_bytes).into()
    }

    /// Sets the prime [ModulusKey].
    pub fn set_modulus(&mut self, modulus: &ModulusKey) {
        let key_bytes = modulus.as_inner().to_le_bytes();
        self.buf[index::MODULUS..index::MODULUS_END].copy_from_slice(key_bytes.as_ref());
    }
}

impl_default!(SetModulusCommand);
impl_command_display!(SetModulusCommand);
impl_message_from_buf!(SetModulusCommand);
impl_message_ops!(SetModulusCommand);
impl_command_ops!(SetModulusCommand);
