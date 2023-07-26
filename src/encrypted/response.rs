use rand_chacha::rand_core::RngCore;
#[cfg(not(feature = "std"))]
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

#[cfg(not(feature = "std"))]
use crate::seed;

use crate::{
    impl_default, impl_encrypted_message_ops, impl_message_from_buf, impl_response_display,
    impl_response_ops, len, AesKey, Error, MessageOps, ResponseOps, Result, SequenceCount,
};

use super::{index, WrappedEncryptedMessage};

/// Encrypted - Response (0x7E)
///
/// The encrypted packet is wrapped inside the data field of a standard SSP packet. The
/// encrypted section is constructed from the following fields.
///
/// | STEX | LENGTH | COUNT 0 | COUNT 1 | COUNT 2 | COUNT 3 | DATA 0 | ... | DATA N | PACKING 0 | ... | PACKING N | CRC_L | CRC_H |
/// |:----:|:------:|:-------:|:-------:|:-------:|:-------:|:------:|:---:|:------:|:---------:|:---:|:---------:|:-----:|:-----:|
/// | 0x7E | 0xnn   | 0xnn    | 0xnn    | 0xnn    | 0xnn    | 0xnn   | ... | 0xnn   | 0xnn      | ... | 0xnn      | 0xnn  | 0xnn  |
#[repr(C)]
#[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct EncryptedResponse {
    buf: [u8; len::ENCRYPTED_RESPONSE],
}

impl EncryptedResponse {
    /// Creates a new [EncryptedResponse].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENCRYPTED_RESPONSE],
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
    pub fn set_message_data<M: ResponseOps>(&mut self, message: &M) -> Result<()> {
        let len = message.data_len();

        if message.data().len() != len {
            return Err(Error::InvalidDataLength((len, message.data().len())));
        }

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

    /// Encrypts and consumes the [EncryptedResponse] message.
    ///
    /// Converts the [EncryptedResponse] message into a standard [WrappedEncryptedMessage].
    ///
    /// **Note**: only useful if implementing a device-side binary, and/or testing host-side
    /// functionality.
    pub fn encrypt(mut self, key: &AesKey) -> WrappedEncryptedMessage {
        use crate::aes;

        self.set_packing();
        self.calculate_checksum();

        let mut enc_msg = WrappedEncryptedMessage::new();

        let enc_len = self.len();
        enc_msg.set_data_len(enc_len as u8);

        let cipher_data = enc_msg.data_mut()[1..].as_mut();
        let plain_data = self.encrypt_data();

        if let Err(err) = aes::aes_encrypt_inplace(key.as_ref(), plain_data, cipher_data) {
            log::error!("error encrypting response message: {err}");
        }

        super::increment_sequence_count();

        if let Err(err) = enc_msg.stuff_encrypted_data() {
            log::error!("error stuffing encrypted response message: {err}");
        }

        enc_msg
    }

    /// Decrypts and consumes the [WrappedEncryptedMessage].
    ///
    /// Converts the [WrappedEncryptedMessage] into an [EncryptedResponse].
    pub fn decrypt(key: &AesKey, mut message: WrappedEncryptedMessage) -> Self {
        use crate::aes;

        if let Err(err) = message.unstuff_encrypted_data() {
            log::error!("error unstuffing encrypted response message: {err}");
        }

        let mut dec_msg = Self::new();
        dec_msg.set_data_len(message.data_len().saturating_sub(len::ENCRYPTED_METADATA) as u8);

        // Skip the STEX (0x7E) byte, it's not encrypted/decrypted
        let cipher_data = message.data()[1..].as_ref();
        let plain_data = dec_msg.encrypt_data();

        if let Err(err) = aes::aes_decrypt_inplace(key.as_ref(), cipher_data, plain_data) {
            log::error!("error decrypting response message: {err}");
        }

        super::increment_sequence_count();

        dec_msg
    }
}

impl_default!(EncryptedResponse);
impl_response_display!(EncryptedResponse);
impl_message_from_buf!(EncryptedResponse);
impl_encrypted_message_ops!(EncryptedResponse);
impl_response_ops!(EncryptedResponse);
