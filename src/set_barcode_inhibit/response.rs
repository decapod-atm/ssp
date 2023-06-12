use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len, MessageOps, MessageType,
};

/// SetBarcodeInhibit - Response (0x25)
///
/// Represents a response to an [SetBarcodeInhibitCommand](crate::SetBarcodeInhibitCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetBarcodeInhibitResponse {
    buf: [u8; len::SET_BARCODE_INHIBIT_RESPONSE],
}

impl SetBarcodeInhibitResponse {
    /// Creates a new [SetBarcodeInhibitResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SET_BARCODE_INHIBIT_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(SetBarcodeInhibitResponse);
impl_message_from_buf!(SetBarcodeInhibitResponse);
impl_message_ops!(SetBarcodeInhibitResponse, MessageType::SetBarcodeInhibit);
impl_response_display!(SetBarcodeInhibitResponse);
impl_response_ops!(SetBarcodeInhibitResponse);
