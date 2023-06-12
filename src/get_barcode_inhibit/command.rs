use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// GetBarcodeInhibit - Command (0x25)
///
/// Single byte command causes validator to return the current bar code/currency inhibit status.
///
/// This indicates whether the validator can accept only currency, only barcodes, both or neither.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeInhibitCommand {
    buf: [u8; len::GET_BARCODE_INHIBIT_COMMAND],
}

impl GetBarcodeInhibitCommand {
    /// Creates a new [GetBarcodeInhibitCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_INHIBIT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::GetBarcodeInhibit);

        msg
    }
}

impl_default!(GetBarcodeInhibitCommand);
impl_command_display!(GetBarcodeInhibitCommand);
impl_message_from_buf!(GetBarcodeInhibitCommand);
impl_message_ops!(GetBarcodeInhibitCommand);
impl_command_ops!(GetBarcodeInhibitCommand);
