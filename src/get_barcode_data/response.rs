use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len, std::fmt,
    BarcodeTicketStatus, MessageOps, MessageType, ResponseOps,
};

mod index {
    pub const STATUS: usize = 4;
    pub const DATA_LEN: usize = 5;
    pub const DATA: usize = 6;
}

/// GetBarcodeData - Response (0x27)
///
/// Represents a response to an [GetBarcodeDataCommand](crate::GetBarcodeDataCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetBarcodeDataResponse {
    buf: [u8; len::GET_BARCODE_DATA_RESPONSE],
}

impl GetBarcodeDataResponse {
    /// Creates a new [GetBarcodeDataResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::GET_BARCODE_DATA_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [BarcodeTicketStatus].
    pub fn ticket_status(&self) -> BarcodeTicketStatus {
        self.buf[index::STATUS].into()
    }

    /// Gets the length of the barcode data.
    pub fn barcode_data_len(&self) -> usize {
        self.buf[index::DATA_LEN] as usize
    }

    /// Gets the barcode data.
    pub fn barcode_data(&self) -> &[u8] {
        let data_end = index::DATA + self.barcode_data_len();
        self.buf[index::DATA..data_end].as_ref()
    }
}

impl_default!(GetBarcodeDataResponse);
impl_message_from_buf!(GetBarcodeDataResponse);
impl_var_message_ops!(GetBarcodeDataResponse, MessageType::GetBarcodeData);
impl_response_ops!(GetBarcodeDataResponse);

impl fmt::Display for GetBarcodeDataResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let ticket_status = self.ticket_status();
        let data = self.barcode_data();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Barcode ticket status: {ticket_status} | Barcode data: {data:x?} | CRC-16: 0x{crc:04x}")
    }
}
