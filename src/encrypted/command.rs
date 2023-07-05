use rand_chacha::rand_core::RngCore;
#[cfg(not(feature = "std"))]
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

#[cfg(not(feature = "std"))]
use crate::seed;

use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_encrypted_message_ops,
    impl_message_from_buf, len, AesBlock, AesKey, CommandOps, Error, MessageOps, Result,
    SequenceCount,
};

use super::{index, WrappedEncryptedMessage};

/// Encrypted - Command (0x7E)
///
/// The encrypted packet is wrapped inside the data field of a standard SSP packet. The
/// encrypted section is constructed from the following fields.
///
/// | STEX | LENGTH | COUNT 0 | COUNT 1 | COUNT 2 | COUNT 3 | DATA 0 | ... | DATA N | PACKING 0 | ... | PACKING N | CRC_L | CRC_H |
/// |:----:|:------:|:-------:|:-------:|:-------:|:-------:|:------:|:---:|:------:|:---------:|:---:|:---------:|:-----:|:-----:|
/// | 0x7E | 0xnn   | 0xnn    | 0xnn    | 0xnn    | 0xnn    | 0xnn   | ... | 0xnn   | 0xnn      | ... | 0xnn      | 0xnn  | 0xnn  |
#[repr(C)]
#[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct EncryptedCommand {
    buf: [u8; len::ENCRYPTED_COMMAND],
}

impl EncryptedCommand {
    /// Creates a new [EncryptedCommand].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENCRYPTED_COMMAND],
        };

        msg.init();

        msg
    }

    /// Gets the sequence count.
    pub fn count(&self) -> SequenceCount {
        self.count_buf().into()
    }

    fn count_buf(&self) -> &[u8] {
        self.buf[index::COUNT..index::COUNT_END].as_ref()
    }

    /// Gets the message data.
    pub fn message_data(&self) -> &[u8] {
        let start = self.data_start();
        let end = self.data_end();

        self.buf[start..end].as_ref()
    }

    fn data_start(&self) -> usize {
        index::DATA
    }

    fn data_end(&self) -> usize {
        self.data_start() + self.data_len()
    }

    /// Sets the message data.
    ///
    /// Returns `Err(_)` if the message data exceeds the maximum length.
    ///
    /// Maximum length: [MAX_ENCRYPTED_DATA](crate::len::MAX_ENCRYPTED_DATA)
    ///
    /// This maximum refers to the data field of the wrapped inner message.
    ///
    /// Encrypted messages wrap the data field of a standard SSP message, and then the entire
    /// encrypted message is wrapped in an outer standard SSP message.
    ///
    /// Matryoshka dolls all the way down...
    pub fn set_message_data(&mut self, message: &mut dyn CommandOps) -> Result<()> {
        let len = message.data_len();

        if message.data().len() != len {
            return Err(Error::InvalidDataLength((len, message.data().len())));
        }

        message.calculate_checksum();

        if (0..=len::MAX_ENCRYPTED_DATA).contains(&len) {
            self.buf[index::LEN] = len as u8;

            let start = self.data_start();
            let end = self.data_end();

            self.buf[start..end].copy_from_slice(message.data());

            Ok(())
        } else {
            Err(Error::InvalidDataLength((len, len::MAX_ENCRYPTED_DATA)))
        }
    }

    /// Gets random packing data used to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    pub fn packing(&self) -> &[u8] {
        let start = self.packing_start();
        let end = self.packing_end();

        self.buf[start..end].as_ref()
    }

    /// Adds random packing data to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    #[cfg(not(feature = "std"))]
    pub fn set_packing(&mut self) {
        if self.packing_len() == 0 {
            return;
        }

        // Less robust than using rand::thread_rng, but still better than PKCS#7 padding...
        let mut rng = ChaCha20Rng::from_seed(seed(self.buf(), self.count_buf()));

        let start = self.packing_start();
        let end = self.packing_end();

        rng.fill_bytes(&mut self.buf[start..end]);
    }

    /// Adds random packing data to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    #[cfg(feature = "std")]
    pub fn set_packing(&mut self) {
        if self.packing_len() == 0 {
            return;
        }

        let mut rng = rand::thread_rng();

        let start = self.packing_start();
        let end = self.packing_end();

        rng.fill_bytes(&mut self.buf[start..end]);
    }

    fn packing_start(&self) -> usize {
        self.data_end()
    }

    fn packing_end(&self) -> usize {
        self.packing_start() + self.packing_len()
    }

    /// Gets the length of packing bytes needed to make the packet's encrypted data a multiple of
    /// the [AES block length](crate::len::AES).
    pub fn packing_len(&self) -> usize {
        // count all metadata bytes except STEX
        let meta = len::METADATA - 1;
        let raw_len = meta + self.data_len();

        len::aes_packing_len(raw_len)
    }

    fn encrypt_data(&mut self) -> &mut [u8] {
        let len = self.len();
        self.buf[index::LEN..len].as_mut()
    }

    /// Encrypts and consumes the [EncryptedCommand] message.
    ///
    /// Converts the [EncryptedCommand] message into a standard [WrappedEncryptedMessage].
    pub fn encrypt(mut self, key: &AesKey) -> WrappedEncryptedMessage {
        use aes::cipher::{BlockEncrypt, KeyInit};

        let aes = aes::Aes128::new(key);

        self.set_packing();
        self.calculate_checksum();

        let mut enc_msg = WrappedEncryptedMessage::new();

        let enc_len = self.len();
        enc_msg.set_data_len(enc_len as u8);

        let enc_chunks = enc_msg.data_mut()[1..].chunks_exact_mut(len::AES);
        let msg_chunks = self.encrypt_data().chunks_exact(len::AES);

        for (msg_block, wrap_block) in msg_chunks.zip(enc_chunks) {
            aes.encrypt_block_b2b(
                AesBlock::from_slice(msg_block),
                AesBlock::from_mut_slice(wrap_block),
            );
        }

        super::increment_sequence_count();

        enc_msg
    }

    /// Decrypts and consumes the [WrappedEncryptedMessage].
    ///
    /// Converts the [WrappedEncryptedMessage] into an [EncryptedCommand].
    ///
    /// **Note**: only useful if implmenting a device-side binary.
    pub fn decrypt(key: &AesKey, message: WrappedEncryptedMessage) -> Self {
        use aes::cipher::{BlockDecrypt, KeyInit};

        let aes = aes::Aes128::new(key);

        let mut dec_msg = Self::new();
        dec_msg.set_data_len(message.data_len().saturating_sub(len::ENCRYPTED_METADATA) as u8);

        // Skip the STEX (0x7E) byte, it's not encrypted/decrypted
        let enc_chunks = message.data()[1..].chunks_exact(len::AES);
        let dec_chunks = dec_msg.encrypt_data().chunks_exact_mut(len::AES);

        for (wrap_block, dec_block) in enc_chunks.zip(dec_chunks) {
            aes.decrypt_block_b2b(
                AesBlock::from_slice(wrap_block),
                AesBlock::from_mut_slice(dec_block),
            );
        }

        super::increment_sequence_count();

        dec_msg
    }
}

impl_default!(EncryptedCommand);
impl_command_display!(EncryptedCommand);
impl_message_from_buf!(EncryptedCommand);
impl_encrypted_message_ops!(EncryptedCommand);
impl_command_ops!(EncryptedCommand);
