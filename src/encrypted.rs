//! Messages for encrypted SSP (eSSP) communication.

use crate::std::sync::atomic::{AtomicU32, Ordering};
use crate::SequenceCount;

static SEQUENCE_COUNT: AtomicU32 = AtomicU32::new(0);

/// Represents the start of an encrypted packet.
pub const STEX: u8 = 0x7e;

mod command;
mod response;
mod wrapped;

pub use command::*;
pub use response::*;
pub use wrapped::*;

pub mod index {
    pub const STEX: usize = 0;
    pub const LEN: usize = 1;
    pub const COUNT: usize = 2;
    pub const COUNT_END: usize = 6;
    pub const DATA: usize = 6;
    pub const COMMAND: usize = 6;
    pub const RESPONSE_STATUS: usize = 6;
}

/// Gets the current [SequenceCount].
pub fn sequence_count() -> SequenceCount {
    SEQUENCE_COUNT.load(Ordering::Relaxed).into()
}

/// Increments the [SequenceCount].
///
/// Returns the new [SequenceCount].
pub fn increment_sequence_count() -> SequenceCount {
    let count = SEQUENCE_COUNT.load(Ordering::Relaxed).saturating_add(1);

    SEQUENCE_COUNT.store(count, Ordering::SeqCst);

    count.into()
}

/// Resets the [SequenceCount] to zero.
///
/// Returns the [SequenceCount] value before the reset.
pub fn reset_sequence_count() -> SequenceCount {
    let count = SEQUENCE_COUNT.load(Ordering::Relaxed);

    SEQUENCE_COUNT.store(0, Ordering::SeqCst);

    count.into()
}

#[cfg(test)]
pub mod tests {
    use crate::{len, AesKey, MessageOps, Result};

    use super::*;

    static TEST_KEY: [u8; len::AES] = [
        0x01, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, b'd', b'e', b'r', b'p', b'd', b'e', b'r',
        b'p',
    ];

    fn test_key() -> AesKey {
        AesKey::clone_from_slice(TEST_KEY.as_ref())
    }

    #[test]
    fn test_command_encryption() -> Result<()> {
        use crate::PollCommand;

        let key = test_key();

        let mut poll_msg = PollCommand::new();

        let mut enc_cmd = EncryptedCommand::new();
        enc_cmd.set_message_data(&mut poll_msg)?;

        let wrap_msg = enc_cmd.clone().encrypt(&key);

        let dec_cmd = EncryptedCommand::decrypt(&key, wrap_msg);

        assert_eq!(dec_cmd.data(), enc_cmd.data());

        dec_cmd.verify_checksum()?;

        Ok(())
    }

    #[test]
    fn test_response_encryption() -> Result<()> {
        use crate::{PollResponse, ResponseOps, ResponseStatus};

        let key = test_key();

        let mut poll_msg = PollResponse::new();
        poll_msg.set_response_status(ResponseStatus::Ok);
        poll_msg.set_data_len(1);

        let mut enc_res = EncryptedResponse::new();
        enc_res.set_message_data(&mut poll_msg)?;

        let wrap_msg = enc_res.clone().encrypt(&key);

        let dec_res = EncryptedResponse::decrypt(&key, wrap_msg);

        assert_eq!(dec_res.data(), enc_res.data());

        dec_res.verify_checksum()?;

        Ok(())
    }
}
