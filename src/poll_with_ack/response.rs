use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len,
    message::index, std::fmt, MessageOps, MessageType, ResponseOps, ResponseStatus,
    ResponseStatusList, Vec,
};

/// PollWithAck - Response (0x56)
///
/// Represents a response to a [PollWithAckCommand](crate::PollWithAckCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PollWithAckResponse {
    buf: [u8; len::POLL_WITH_ACK_RESPONSE],
}

impl PollWithAckResponse {
    /// Creates a new [PollWithAckResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::POLL_WITH_ACK_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [ResponseStatus]es processed since the last [PollWithAckCommand](crate::PollWithAckCommand).
    pub fn last_response_statuses(&self) -> ResponseStatusList {
        let status_end = len::HEADER + self.data_len();

        self.buf[index::DATA..status_end]
            .iter()
            .map(|&s| ResponseStatus::from(s))
            .collect::<Vec<_>>()
            .into()
    }
}

impl_default!(PollWithAckResponse);
impl_message_from_buf!(PollWithAckResponse);
impl_var_message_ops!(PollWithAckResponse, MessageType::PollWithAck);
impl_response_ops!(PollWithAckResponse);

impl fmt::Display for PollWithAckResponse {
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
