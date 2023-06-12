//! Generic message types for the library.

use crate::std;
use std::fmt;

use crate::{crc::crc16, len::METADATA, Error, MessageType, ResponseStatus, Result, SequenceId};

mod variant;
pub use variant::*;

/// Single byte indicating the start of a packet, defined as 0x7F. If any other part of the packet
/// contains 0x7F, the last step before transmission the byte should be repeated (0x7F becomes 0x7F 0x7F)
/// to indicate it is not a STX byte; this is called byte stuffing.
pub const STX: u8 = 0x7f;
pub const STEX: u8 = 0x7f;
pub const STEXN: u8 = 0x7e;

/// Common indices in message buffers.
pub mod index {
    pub const STX: usize = 0;
    pub const SEQ_ID: usize = 1;
    pub const LEN: usize = 2;
    pub const DATA: usize = 3;
    pub const ENCRYPTED_DATA: usize = 5;
    pub const COMMAND: usize = 3;
    pub const ENCRYPTED_COMMAND: usize = 5;
    pub const RESPONSE_STATUS: usize = 3;
    pub const ENCRYPTED_RESPONSE_STATUS: usize = 5;
}

/// Generic message functions common to all message types.
pub trait MessageOps {
    /// Gets a reference to the message buffer.
    fn buf(&self) -> &[u8];

    /// Gets a mutable reference to the message buffer.
    fn buf_mut(&mut self) -> &mut [u8];

    /// Gets the message type.
    fn message_type(&self) -> MessageType;

    /// Gets whether the message is a command message.
    fn is_command(&self) -> bool;

    /// Gets whether the message is a response message.
    fn is_response(&self) -> bool {
        !self.is_command()
    }

    /// Gets whether the message has a variable data length.
    fn is_variable(&self) -> bool {
        false
    }

    /// Gets the full length of the message buffer.
    fn len(&self) -> usize {
        self.metadata_len() + self.data_len()
    }

    /// Gets the length of the data bytes in the message buffer.
    fn data_len(&self) -> usize {
        let buf = self.buf();
        let inited = buf[index::LEN] != 0;

        if inited {
            buf[index::LEN] as usize
        } else {
            buf.len() - self.metadata_len()
        }
    }

    /// Sets the data length field in the message buffer.
    fn set_data_len(&mut self, len: u8) {
        self.buf_mut()[index::LEN] = len;
    }

    /// Gets the length of the metadata fields in the message buffer.
    fn metadata_len(&self) -> usize {
        super::len::METADATA
    }

    /// Gets whether the message contains any data bytes.
    fn is_empty(&self) -> bool {
        self.data_len() == 0
    }

    /// Common field initialization.
    fn init(&mut self) {
        let data_len = self.data_len();
        let buf = self.buf_mut();

        buf[index::STX] = STX;
        buf[index::SEQ_ID] = SequenceId::new().into();
        buf[index::LEN] = data_len as u8;
    }

    /// Gets the STX byte.
    fn stx(&self) -> u8 {
        self.buf()[index::STX]
    }

    /// Gets the data length field.
    fn length(&self) -> u8 {
        self.buf()[index::LEN]
    }

    /// Gets the [SequenceId] field.
    fn sequence_id(&self) -> SequenceId {
        let buf = self.buf();

        buf[index::SEQ_ID].into()
    }

    /// Sets the [SequenceId] field.
    fn set_sequence_id(&mut self, id: SequenceId) {
        let buf = self.buf_mut();

        buf[index::SEQ_ID] = id.into();
    }

    /// Toggles the [SequenceFlag](crate::SequenceFlag) value of the [SequenceId].
    fn toggle_sequence_id(&mut self) {
        self.set_sequence_id(!self.sequence_id());
    }

    /// Gets a reference to the data field.
    fn data(&self) -> &[u8] {
        let start = index::DATA;
        let end = start + self.data_len();

        self.buf()[start..end].as_ref()
    }

    /// Gets a mutable reference to the data field.
    fn data_mut(&mut self) -> &mut [u8] {
        let start = index::DATA;
        let end = start + self.data_len();

        self.buf_mut()[start..end].as_mut()
    }

    /// Toggles the value of the [SequenceFlag](crate::SequenceFlag).
    fn toggle_sequence_flag(&mut self) {
        let mut seq_id = SequenceId::from(self.buf()[index::SEQ_ID]);

        seq_id.toggle_flag();

        self.buf_mut()[index::SEQ_ID] = seq_id.into();
    }

    /// Gets the checksum from the message buffer.
    fn checksum(&self) -> u16 {
        let buf = self.buf();
        let len = self.len();

        u16::from_le_bytes([buf[len - 2], buf[len - 1]])
    }

    /// Calculates the CRC-16 checksum of the message.
    fn calculate_checksum(&mut self) -> u16 {
        let len = self.len();
        let buf = self.buf_mut();

        let crc = crc16(buf[index::SEQ_ID..len - 2].as_ref());
        buf[len - 2..len].copy_from_slice(crc.to_le_bytes().as_ref());

        crc
    }

    /// Verifies the checksum in the message buffer matches the calculated value.
    fn verify_checksum(&self) -> Result<()> {
        let buf = self.buf();
        let len = self.len();

        let crc = self.checksum();
        let exp_crc = crc16(buf[index::SEQ_ID..len - 2].as_ref());

        if crc == exp_crc {
            Ok(())
        } else {
            Err(Error::Crc((crc, exp_crc)))
        }
    }

    /// Gets the message as a byte buffer.
    ///
    /// Calculates and sets the CRC-16 checksum.
    fn as_bytes(&mut self) -> &[u8] {
        self.calculate_checksum();

        self.buf()
    }

    /// Gets the message as a mutable byte buffer.
    ///
    /// Calculates and sets the CRC-16 checksum.
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.calculate_checksum();

        self.buf_mut()
    }

    /// Constructs a message from a byte buffer.
    fn from_buf(&mut self, buf: &[u8]) -> Result<()> {
        let len = self.len();
        let mut buf_len = buf.len();
        let is_variable = self.is_variable();

        if !is_variable && buf_len < len {
            return Err(Error::InvalidLength((buf_len, len)));
        }

        let stx = buf[index::STX];
        if stx != STX {
            return Err(Error::InvalidSTX(stx));
        }

        let buf_data_len = buf[index::LEN] as usize;

        if !is_variable {
            let data_len = self.data_len();
            if buf_data_len != data_len {
                return Err(Error::InvalidDataLength((buf_data_len, data_len)));
            }
        }

        let msg_type = self.message_type();
        if self.is_command() && !self.is_response() {
            let raw_type = buf[index::COMMAND];
            let buf_msg_type = MessageType::from(raw_type);

            if buf_msg_type != msg_type {
                return Err(Error::InvalidMessageRaw((buf_msg_type, raw_type)));
            }
        }

        log::trace!("message type: {msg_type}, buffer: {buf:x?}");

        buf_len = std::cmp::min(buf_len, buf_data_len + METADATA);
        let buf_crc = u16::from_le_bytes(buf[buf_len - 2..].try_into().unwrap());
        let exp_crc = crc16(buf[index::SEQ_ID..buf_len - 2].as_ref());

        if buf_crc != exp_crc {
            return Err(Error::Crc((buf_crc, exp_crc)));
        }

        self.buf_mut()[..buf_len].copy_from_slice(buf[..buf_len].as_ref());

        Ok(())
    }
}

impl fmt::Display for &dyn MessageOps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "STX: 0x{:02x} SEQID: {} Data: {:x?} CRC-16: 0x{:04x}",
            self.stx(),
            self.sequence_id(),
            self.data(),
            self.checksum(),
        )
    }
}

/// Generic functions for Command type messages.
pub trait CommandOps: MessageOps {
    /// Gets the Command message type.
    fn command(&self) -> MessageType {
        let data = self.data();

        if data.is_empty() {
            MessageType::default()
        } else {
            data[0].into()
        }
    }

    /// Sets the Command message type.
    fn set_command(&mut self, command: MessageType) {
        let data = self.data_mut();

        if !data.is_empty() {
            data[0] = command.into();
        }
    }
}

impl fmt::Display for &dyn CommandOps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let command = self.command();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Command: {command} | CRC-16: 0x{crc:04x}")
    }
}

/// Generic functions for Response type messages.
pub trait ResponseOps: MessageOps {
    /// Gets the Response status of the message.
    fn response_status(&self) -> ResponseStatus {
        let data = self.data();

        if data.is_empty() {
            ResponseStatus::default()
        } else {
            data[0].into()
        }
    }

    /// Sets the Response status of the message.
    fn set_response_status(&mut self, status: ResponseStatus) {
        self.buf_mut()[index::RESPONSE_STATUS] = status.into();
    }
}

impl fmt::Display for &dyn ResponseOps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | CRC-16: 0x{crc:04x}")
    }
}
