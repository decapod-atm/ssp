//! Messages for encrypted SSP (eSSP) communication.

use crate::std::{
    cmp,
    sync::atomic::{AtomicU32, Ordering},
};
use crate::{Error, Result, SequenceCount, STEX};

static SEQUENCE_COUNT: AtomicU32 = AtomicU32::new(0);

mod command;
mod response;
mod wrapped;

pub use command::*;
pub use response::*;
pub use wrapped::*;

pub mod encrypted_index {
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

/// Stuffs encrypted buffers with repeated `STX` bytes if they occur.
///
/// Because encryption is pseudo-random, it is possible for `STX(0x7f)` to appear in encrypted
/// data. Device firmware has an issue with this, because it thinks a new packet is starting.
///
/// ITL solves this by repeating `STX` bytes instead of ignoring them until a packet is parsed...
pub fn stuff(buf: &mut [u8], mut end: usize) -> Result<usize> {
    use crate::message::STX;

    let len = buf.len();

    let mut i = 0;
    while i <= end {
        match end.cmp(&len) {
            cmp::Ordering::Equal => return Ok(end),
            cmp::Ordering::Greater => return Err(Error::InvalidLength((end, len))),
            _ => (),
        }

        if buf[i] == STX {
            // copy all bytes forward a position
            // e.g. convert
            //   0x7f 0xaa 0xbb
            //   0x7f 0x7f 0xaa 0xbb
            //
            // need to go in reverse order to avoid overwriting copied bytes
            for rep in ((i + 1)..=(end + 1)).rev() {
                buf[rep] = buf[rep - 1];
            }

            // skip two indices to avoid copying a stuffed `STX` byte
            i += 2;
            end += 1;
        } else {
            i += 1;
        }
    }

    Ok(end)
}

/// Removes byte stuffing from an encrypted buffer, i.e. repeated `STX(0x7f)` bytes.
///
/// This function should be called before attempting to decrypt an encrypted device reply.
///
/// [EncryptedResponse::decrypt] handles this internally, and users should prefer using this
/// function call.
pub fn unstuff(buf: &mut [u8], mut end: usize) -> Result<usize> {
    use crate::message::STX;

    let len = buf.len();

    let mut i = end;

    while i > 0 {
        match end.cmp(&len) {
            cmp::Ordering::Equal => return Ok(end),
            cmp::Ordering::Greater => return Err(Error::InvalidLength((end, len))),
            _ => (),
        }

        if buf[i] == STX && buf[i - 1] == STX {
            // overwrite the stuffed `STX` byte
            // shift all trailing bytes left one position
            for rep in (i - 1)..end {
                buf[rep] = buf[rep + 1];
            }

            // zero-out the previous trailing byte
            buf[end] = 0;

            i = i.saturating_sub(2);
            end = end.saturating_sub(1);
        } else {
            i = i.saturating_sub(1);
        }
    }

    Ok(end)
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

    #[test]
    fn test_byte_stuffing() -> Result<()> {
        let mut buf = [0x7f, 0xaa, 0xbb, 0x00];
        let exp = [0x7f, 0x7f, 0xaa, 0xbb];
        let end = 2;
        let exp_end = 3;

        let new_end = stuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        // double stuff
        let mut buf = [0x7f, 0xaa, 0x7f, 0x00, 0x00];
        let exp = [0x7f, 0x7f, 0xaa, 0x7f, 0x7f];
        let end = 2;
        let exp_end = 4;

        let new_end = stuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        // triple stuff
        let mut buf = [0x7f, 0xaa, 0x7f, 0xbb, 0x00, 0x00];
        let exp = [0x7f, 0x7f, 0xaa, 0x7f, 0x7f, 0xbb];
        let end = 3;
        let exp_end = 5;

        let new_end = stuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        Ok(())
    }

    #[test]
    fn test_byte_unstuffing() -> Result<()> {
        let mut buf = [0x7f, 0x7f, 0xaa, 0xbb];
        let exp = [0x7f, 0xaa, 0xbb, 0x00];
        let end = buf.len() - 1;
        let exp_end = 2;

        let new_end = unstuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        // double stuff
        let mut buf = [0x7f, 0x7f, 0xaa, 0x7f, 0x7f];
        let exp = [0x7f, 0xaa, 0x7f, 0x00, 0x00];
        let end = buf.len() - 1;
        let exp_end = 2;

        let new_end = unstuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        // triple stuff
        let mut buf = [0x7f, 0x7f, 0xaa, 0x7f, 0x7f, 0xbb];
        let exp = [0x7f, 0xaa, 0x7f, 0xbb, 0x00, 0x00];
        let end = buf.len() - 1;
        let exp_end = 3;

        let new_end = unstuff(buf.as_mut(), end)?;

        assert_eq!(buf, exp);
        assert_eq!(new_end, exp_end);

        Ok(())
    }
}
