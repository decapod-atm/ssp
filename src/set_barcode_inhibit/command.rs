use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, BarcodeCurrencyInhibit, CommandOps, MessageOps, MessageType,
};

mod index {
    pub const INHIBIT: usize = 4;
}

/// SetBarcodeInhibit - Command (0x26)
///
/// Two byte command sets bar code/currency inhibits. When the unit is started up or reset,
/// the default is currency enabled, bar code disabled (0xFE).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetBarcodeInhibitCommand {
    buf: [u8; len::SET_BARCODE_INHIBIT_COMMAND],
}

impl SetBarcodeInhibitCommand {
    /// Creates a new [SetBarcodeInhibitCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_BARCODE_INHIBIT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::SetBarcodeInhibit);

        msg
    }

    /// Gets the [BarcodeCurrencyInhibit] setting.
    pub fn inhibit(&self) -> BarcodeCurrencyInhibit {
        self.buf[index::INHIBIT].into()
    }

    /// Sets the [BarcodeCurrencyInhibit] setting.
    pub fn set_inhibit(&mut self, inhibit: BarcodeCurrencyInhibit) {
        self.buf[index::INHIBIT] = inhibit.into();
    }
}

impl_default!(SetBarcodeInhibitCommand);
impl_command_display!(SetBarcodeInhibitCommand);
impl_message_from_buf!(SetBarcodeInhibitCommand);
impl_message_ops!(SetBarcodeInhibitCommand);
impl_command_ops!(SetBarcodeInhibitCommand);
