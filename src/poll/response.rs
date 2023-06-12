use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len,
    message::index, std::fmt, MessageOps, MessageType, ResponseOps, ResponseStatus,
    ResponseStatusList, Vec,
};

/// Poll - Response (0x7F)
///
/// Represents a response to a standard [PollCommand](crate::PollCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PollResponse {
    buf: [u8; len::POLL_RESPONSE],
}

impl PollResponse {
    /// Creates a new [PollResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::POLL_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [ResponseStatus]es processed since the last [PollCommand](crate::PollCommand).
    pub fn last_response_statuses(&self) -> ResponseStatusList {
        let status_end = len::HEADER + self.data_len();

        self.buf[index::DATA..status_end]
            .iter()
            .map(|&s| ResponseStatus::from(s))
            .collect::<Vec<_>>()
            .into()
    }
}

impl_default!(PollResponse);
impl_message_from_buf!(PollResponse);
impl_var_message_ops!(PollResponse, MessageType::Poll);
impl_response_ops!(PollResponse);

impl fmt::Display for PollResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let statuses = self.last_response_statuses();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Last response statuses: {statuses} | CRC-16: 0x{crc:04x}")
    }
}
