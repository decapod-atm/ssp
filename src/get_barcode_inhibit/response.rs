use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_ops, len, std::fmt,
    BarcodeCurrencyInhibit, MessageOps, MessageType, ResponseOps,
};

mod index {
    pub const INHIBIT: usize = 4;
}

/// GetBarcodeInhibit - Response (0x25)
///
/// Represents a response to an [GetBarcodeInhibitCommand](crate::GetBarcodeInhibitCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeInhibitResponse {
    buf: [u8; len::GET_BARCODE_INHIBIT_RESPONSE],
}

impl GetBarcodeInhibitResponse {
    /// Creates a new [GetBarcodeInhibitResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_INHIBIT_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [BarcodeCurrencyInhibit].
    pub fn inhibit(&self) -> BarcodeCurrencyInhibit {
        self.buf[index::INHIBIT].into()
    }
}

impl_default!(GetBarcodeInhibitResponse);
impl_message_from_buf!(GetBarcodeInhibitResponse);
impl_message_ops!(GetBarcodeInhibitResponse, MessageType::GetBarcodeInhibit);
impl_response_ops!(GetBarcodeInhibitResponse);

impl fmt::Display for GetBarcodeInhibitResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let inhibit = self.inhibit();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | {inhibit} | CRC-16: 0x{crc:04x}")
    }
}
