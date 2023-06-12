use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// GetBarcodeData - Command (0x27)
///
/// Single byte command causes validator to return the last valid barcode ticket data.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeDataCommand {
    buf: [u8; len::GET_BARCODE_INHIBIT_COMMAND],
}

impl GetBarcodeDataCommand {
    /// Creates a new [GetBarcodeDataCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_INHIBIT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::GetBarcodeData);

        msg
    }
}

impl_default!(GetBarcodeDataCommand);
impl_command_display!(GetBarcodeDataCommand);
impl_message_from_buf!(GetBarcodeDataCommand);
impl_message_ops!(GetBarcodeDataCommand);
impl_command_ops!(GetBarcodeDataCommand);
