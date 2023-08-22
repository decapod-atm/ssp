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

/// Sets the current [SequenceCount].
pub fn set_sequence_count(count: u32) {
    SEQUENCE_COUNT.store(count, Ordering::SeqCst);
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

#[cfg(all(test, feature = "std"))]
pub mod tests {
    use crate::{len, std::sync::Mutex, AesKey, MessageOps, Result};

    use super::*;

    static TEST_KEY: [u8; len::AES] = [
        0x01, 0x23, 0x45, 0x67, 0x01, 0x23, 0x45, 0x67, b'd', b'e', b'r', b'p', b'd', b'e', b'r',
        b'p',
    ];

    // Add a mutex to run tests one at a time
    static T: Mutex<()> = Mutex::new(());

    fn test_key() -> AesKey {
        AesKey::clone_from_slice(TEST_KEY.as_ref())
    }

    #[test]
    fn test_command_encryption() -> Result<()> {
        use crate::PollCommand;

        let _lock = T.lock();

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

        let _lock = T.lock();

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
        let _lock = T.lock();

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
        let _lock = T.lock();

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

    #[test]
    fn test_encrypt_decrypt_stuffing() -> Result<()> {
        use crate::PollCommand;

        let _lock = T.lock();

        let mut msg = PollCommand::new();
        let _clear_csum = msg.calculate_checksum();

        let key = AesKey::from([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14,
            0x15, 0x16,
        ]);

        #[cfg(feature = "std")]
        println!("Clear-text command buffer: {:x?}", msg.buf());

        let mut enc_msg = EncryptedCommand::new();

        enc_msg.set_message_data(&mut msg)?;
        let wrap_msg = enc_msg.encrypt(&key);

        let _wrap_csum = wrap_msg.checksum();

        #[cfg(feature = "std")]
        println!("Wrapped encrypted command buffer: {:x?}", wrap_msg.buf());

        let dec_msg = EncryptedResponse::decrypt(&key, wrap_msg);

        dec_msg.verify_checksum()?;

        Ok(())
    }

    #[test]
    fn test_encrypt_known_keys() -> Result<()> {
        use crate::{HostProtocolVersionCommand, ProtocolVersion};

        let _lock = T.lock();

        let key = AesKey::from([
            0x67, 0x45, 0x23, 0x01, 0x67, 0x45, 0x23, 0x01, 0x5e, 0xfa, 0xe5, 0x0d, 0x00, 0x00,
            0x00, 0x00,
        ]);

        let exp_enc_bytes = [
            0x7f, 0x80, 0x11, 0x7e, 0x86, 0x01, 0xf3, 0xa7, 0x85, 0xec, 0x6f, 0x4b, 0x3d, 0x0f,
            0xf0, 0xfe, 0xd4, 0x8d, 0x16, 0x64, 0x2a, 0x23,
        ];

        let msg = HostProtocolVersionCommand::new().with_version(ProtocolVersion::Six);

        set_sequence_count(0);

        let mut wrap_msg = EncryptedCommand::new()
            .with_count(0u32.into())
            .with_message_data(&msg)?
            .encrypt(&key);

        wrap_msg.set_sequence_id(128u8.into());
        wrap_msg.calculate_checksum();

        assert_eq!(wrap_msg.buf(), exp_enc_bytes.as_ref());

        wrap_msg.verify_checksum()
    }
}
