use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops,
    len, CommandOps, MessageOps, MessageType,
};

/// GetBarcodeReaderConfiguration - Command (0x24)
///
/// Single byte command causes the validator to return the configuration data for attached barcode
/// readers if there is one present.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeReaderConfigurationCommand {
    buf: [u8; len::GET_BARCODE_READER_CONFIGURATION_COMMAND],
}

impl GetBarcodeReaderConfigurationCommand {
    /// Creates a new [GetBarcodeReaderConfigurationCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_READER_CONFIGURATION_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::GetBarcodeReaderConfiguration);

        msg
    }
}

impl_default!(GetBarcodeReaderConfigurationCommand);
impl_command_display!(GetBarcodeReaderConfigurationCommand);
impl_message_from_buf!(GetBarcodeReaderConfigurationCommand);
impl_message_ops!(GetBarcodeReaderConfigurationCommand);
impl_command_ops!(GetBarcodeReaderConfigurationCommand);
