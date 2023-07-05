use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len,
    std::{self, fmt},
    Error, MessageOps, MessageType, ResponseOps, Result,
};

pub mod index {
    pub const DATASET_VERSION: usize = 4;
}

/// DatasetVersion - Response (0x21)
///
/// Represents a response to a [DatasetVersionCommand](crate::DatasetVersionCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DatasetVersionResponse {
    buf: [u8; len::DATASET_VERSION_RESPONSE],
}

impl DatasetVersionResponse {
    /// Creates a new [DatasetVersionResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::DATASET_VERSION_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [ResponseStatus]es processed since the last [DatasetVersionCommand](crate::DatasetVersionCommand).
    pub fn dataset_version(&self) -> Result<&str> {
        let version_end = len::HEADER + self.data_len();
        let data_end = self.buf.len() - 2;

        if version_end > data_end {
            Err(Error::InvalidLength((version_end, data_end)))
        } else {
            std::str::from_utf8(self.buf[index::DATASET_VERSION..version_end].as_ref())
                .map_err(|err| err.into())
        }
    }
}

impl_default!(DatasetVersionResponse);
impl_message_from_buf!(DatasetVersionResponse);
impl_var_message_ops!(DatasetVersionResponse, MessageType::DatasetVersion);
impl_response_ops!(DatasetVersionResponse);

impl fmt::Display for DatasetVersionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let version = self.dataset_version().unwrap_or("");
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Dataset version: {version} | CRC-16: 0x{crc:04x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_dataset_parsing() -> Result<()> {
        let msg_bytes = [0x7f, 0x00, 0x09, 0xf0, b'E', b'U', b'R', b'O', b'1', b'6', b'1', b'0', 0xac, 0x9e]; 
        let exp_dataset_version = "EURO1610";
        let msg = DatasetVersionResponse::try_from(msg_bytes.as_ref())?;

        assert_eq!(msg.dataset_version()?, exp_dataset_version);

        Ok(())
    }
}
