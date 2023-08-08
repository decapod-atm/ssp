use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf, impl_response_ops,
    impl_wrapped_message_ops, len, CommandOps, Error, MessageOps, MessageType, Result,
};

/// Wrapped Encrypted Message (0x7E)
///
/// Variable length message that wraps an encrypted SSP (eSSP) message.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct WrappedEncryptedMessage {
    buf: [u8; len::WRAPPED_ENCRYPTED_MESSAGE],
}

impl WrappedEncryptedMessage {
    /// Creates a new [WrappedEncryptedMessage] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::WRAPPED_ENCRYPTED_MESSAGE],
        };

        msg.init();
        msg.set_command(MessageType::Encrypted);

        msg
    }

    pub fn is_encrypted(&self) -> bool {
        use crate::message::STEX;
        self.data()[0] == STEX
    }

    /// Gets the wrapped encrypted data.
    pub fn encrypted_data(&self) -> &[u8] {
        use crate::message::index;

        let start = index::DATA + 1;
        let end = start + self.data_len() - 1;

        self.buf[start..end].as_ref()
    }

    /// Adds byte stuffing to the encrypted message data to avoid devices thinking we started a new
    /// packet in the middle of an encrypted packet.
    pub(crate) fn stuff_encrypted_data(&mut self) -> Result<usize> {
        use super::stuff;
        use crate::message::index;

        let start = index::DATA + 1;
        // end after the CRC data
        let end = start + self.data_len() + 2;

        stuff(&mut self.buf[start..], end)
    }

    /// Removes byte stuffing from the encrypted message data.
    pub(crate) fn unstuff_encrypted_data(&mut self) -> Result<()> {
        use super::unstuff;
        use crate::message::{index, STX};

        let start = index::DATA + 1;
        // end after the CRC data
        let exp_end = start + self.data_len() + 2;
        // Because there may be byte stuffing, the actual end of the data is somewhere between the
        // end of where the CRC should be, and the maximum buffer length.
        //
        // This is because stuffed bytes are not added to the `LEN` byte count.
        let len = self.buf.len();
        let mut i = start;
        let mut end = exp_end;
        while end < len && i < end {
            if self.buf[i] == STX && self.buf[i + 1] == STX {
                i += 2;
                end += 1;
            } else {
                i += 1;
            }
        }

        let calc_end = unstuff(&mut self.buf[start..], end)?;
        if calc_end != exp_end {
            Err(Error::InvalidLength((calc_end, exp_end)))
        } else {
            Ok(())
        }
    }
}

impl_default!(WrappedEncryptedMessage);
impl_command_display!(WrappedEncryptedMessage);
impl_message_from_buf!(WrappedEncryptedMessage);
impl_wrapped_message_ops!(WrappedEncryptedMessage);
impl_command_ops!(WrappedEncryptedMessage);
impl_response_ops!(WrappedEncryptedMessage);
