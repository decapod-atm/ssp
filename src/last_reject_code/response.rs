use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_ops,
    len::LAST_REJECT_CODE_RESPONSE, std::fmt, LastRejectCode, MessageOps, MessageType, ResponseOps,
};

mod index {
    pub const REJECT_CODE: usize = 4;
}

/// LastRejectCode - Response (0x17)
///
/// Represents a response to an [LastRejectCodeCommand](crate::LastRejectCodeCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LastRejectCodeResponse {
    buf: [u8; LAST_REJECT_CODE_RESPONSE],
}

impl LastRejectCodeResponse {
    /// Creates a new [LastRejectCodeResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; LAST_REJECT_CODE_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [LastRejectCode].
    pub fn reject_code(&self) -> LastRejectCode {
        self.buf[index::REJECT_CODE].into()
    }

    /// Sets the [LastRejectCode].
    pub fn set_reject_code(&mut self, code: LastRejectCode) {
        self.buf[index::REJECT_CODE] = code.into();
    }
}

impl_default!(LastRejectCodeResponse);
impl_message_from_buf!(LastRejectCodeResponse);
impl_message_ops!(LastRejectCodeResponse, MessageType::LastRejectCode);
impl_response_ops!(LastRejectCodeResponse);

impl fmt::Display for LastRejectCodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let code = self.reject_code();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Last reject code: {code} | CRC-16: 0x{crc:04x}")
    }
}
