use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, GeneratorKey, MessageOps, MessageType,
};

mod index {
    pub const GENERATOR: usize = 4;
    pub const GENERATOR_END: usize = 12;
}

/// SetGenerator - Command (0x4A)
///
/// Eight data bytes are sent. This is a 64 bit number representing the Generator, and must be a
/// prime number.
///
/// The device will reply with OK or PARAMETER_OUT_OF_RANGE if the number is not prime.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetGeneratorCommand {
    buf: [u8; len::SET_GENERATOR_COMMAND],
}

impl SetGeneratorCommand {
    /// Creates a new [SetGeneratorCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_GENERATOR_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetGenerator);

        msg
    }

    /// Gets the prime [GeneratorKey].
    pub fn generator(&self) -> GeneratorKey {
        let gen_bytes: [u8; 8] = self.buf[index::GENERATOR..index::GENERATOR_END]
            .try_into()
            .unwrap();
        u64::from_le_bytes(gen_bytes).into()
    }

    /// Sets the prime [GeneratorKey].
    pub fn set_generator(&mut self, generator: &GeneratorKey) {
        let gen_bytes = generator.as_inner().to_le_bytes();
        self.buf[index::GENERATOR..index::GENERATOR_END].copy_from_slice(gen_bytes.as_ref());
    }
}

impl_default!(SetGeneratorCommand);
impl_command_display!(SetGeneratorCommand);
impl_message_from_buf!(SetGeneratorCommand);
impl_message_ops!(SetGeneratorCommand);
impl_command_ops!(SetGeneratorCommand);
