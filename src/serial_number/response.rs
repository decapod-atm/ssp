use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_ops,
    len::SERIAL_NUMBER_RESPONSE, std::fmt, MessageOps, MessageType, ResponseOps, SerialNumber,
};

pub mod index {
    pub const SERIAL_NUMBER: usize = 4;
    pub const SERIAL_NUMBER_END: usize = 8;
}

/// SerialNumber - Response (0x0A)
///
/// Represents a response to an [SerialNumberCommand](crate::SerialNumberCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SerialNumberResponse {
    buf: [u8; SERIAL_NUMBER_RESPONSE],
}

impl SerialNumberResponse {
    /// Creates a new [SerialNumberResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; SERIAL_NUMBER_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [SerialNumber].
    pub fn serial_number(&self) -> SerialNumber {
        self.buf[index::SERIAL_NUMBER..index::SERIAL_NUMBER_END]
            .as_ref()
            .into()
    }

    /// Sets the [SerialNumber].
    pub fn set_serial_number(&mut self, serial_number: SerialNumber) {
        self.buf[index::SERIAL_NUMBER..index::SERIAL_NUMBER_END]
            .copy_from_slice(serial_number.as_inner().to_be_bytes().as_ref());
    }
}

impl_default!(SerialNumberResponse);
impl_message_from_buf!(SerialNumberResponse);
impl_message_ops!(SerialNumberResponse, MessageType::SerialNumber);
impl_response_ops!(SerialNumberResponse);

impl fmt::Display for SerialNumberResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let data_len = self.data_len();
        let status = self.response_status();
        let serial_number = self.serial_number();
        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{data_len:02x} | Response status: {status} | Serial Number: {serial_number}")
    }
}
