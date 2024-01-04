//! Lengths of various messages (full-byte length, not data length).

/// Reset Command full message length.
pub const RESET_COMMAND: usize = 6;
/// SetInhibits Command full message length.
///
/// Because SetInhibits command messages are variable, set the static length to the maximum valid
/// number of channel bytes:
///
/// ```no_build,no_run
/// 3 header bytes + 1 command byte + 8 channel bytes + 2 footer bytes
/// ```
pub const SET_INHIBITS_COMMAND: usize = 14;
/// SetInhibits Response full message length.
pub const SET_INHIBITS_RESPONSE: usize = 6;
/// EncryptionReset Command full message length.
pub const ENCRYPTION_RESET_COMMAND: usize = 6;
/// EncryptionReset Response full message length.
pub const ENCRYPTION_RESET_RESPONSE: usize = 6;
/// GetBarcodeInhibit Command full message length.
pub const GET_BARCODE_INHIBIT_COMMAND: usize = 6;
/// GetBarcodeInhibit Response full message length.
pub const GET_BARCODE_INHIBIT_RESPONSE: usize = 7;
/// GetBarcodeData Command full message length.
pub const GET_BARCODE_DATA_COMMAND: usize = 6;
/// GetBarcodeData Response full message length.
pub const GET_BARCODE_DATA_RESPONSE: usize = MAX_MESSAGE;
/// SetBarcodeInhibit Command full message length.
pub const SET_BARCODE_INHIBIT_COMMAND: usize = 7;
/// SetBarcodeInhibit Response full message length.
pub const SET_BARCODE_INHIBIT_RESPONSE: usize = 6;
/// GetBarcodeReaderConfiguration Command full message length.
pub const GET_BARCODE_READER_CONFIGURATION_COMMAND: usize = 6;
/// GetBarcodeReaderConfiguration Response full message length.
pub const GET_BARCODE_READER_CONFIGURATION_RESPONSE: usize = 10;
/// SetBarcodeReaderConfiguration Command full message length.
pub const SET_BARCODE_READER_CONFIGURATION_COMMAND: usize = 9;
/// SetBarcodeReaderConfiguration Response full message length.
pub const SET_BARCODE_READER_CONFIGURATION_RESPONSE: usize = 10;
/// Hold Command full message length.
pub const HOLD_COMMAND: usize = 6;
/// Hold Response full message length.
pub const HOLD_RESPONSE: usize = 6;
/// Host Protocol Version Command full message length.
pub const HOST_PROTOCOL_VERSION_COMMAND: usize = 7;
/// Host Protocol Version Response full message length.
pub const HOST_PROTOCOL_VERSION_RESPONSE: usize = 6;
/// Sync Command full message length.
pub const SYNC_COMMAND: usize = 6;
/// Sync Response full message length.
pub const SYNC_RESPONSE: usize = 6;
/// Display On Command full message length.
pub const DISPLAY_ON_COMMAND: usize = 6;
/// Display On Response full message length.
pub const DISPLAY_ON_RESPONSE: usize = 6;
/// Display Off Command full message length.
pub const DISPLAY_OFF_COMMAND: usize = 6;
/// Display Off Response full message length.
pub const DISPLAY_OFF_RESPONSE: usize = 6;
/// Reject Command full message length.
pub const REJECT_COMMAND: usize = 6;
/// Reject Response full message length.
pub const REJECT_RESPONSE: usize = 6;
/// Poll Command full message length.
pub const POLL_COMMAND: usize = 6;
/// Poll Response maximum full message length.
///
/// Because poll messages have variable response lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const POLL_RESPONSE: usize = MAX_MESSAGE;
/// Disable Command full message length.
pub const DISABLE_COMMAND: usize = 6;
/// Disable Response full message length.
pub const DISABLE_RESPONSE: usize = 6;
/// Enable Command full message length.
pub const ENABLE_COMMAND: usize = 6;
/// Enable Response full message length.
pub const ENABLE_RESPONSE: usize = 6;
/// Dataset Version Command full message length.
pub const DATASET_VERSION_COMMAND: usize = 6;
/// Dataset Version Response full message length.
pub const DATASET_VERSION_RESPONSE: usize = MAX_MESSAGE;
/// Empty Command full message length.
pub const EMPTY_COMMAND: usize = 6;
/// Empty Response full message length.
pub const EMPTY_RESPONSE: usize = 6;
/// SmartEmpty Command full message length.
pub const SMART_EMPTY_COMMAND: usize = 6;
/// SmartEmpty Response full message length.
pub const SMART_EMPTY_RESPONSE: usize = 6;
/// Serial Number Command full message length.
pub const SERIAL_NUMBER_COMMAND: usize = 6;
/// Serial Number Response full message length.
pub const SERIAL_NUMBER_RESPONSE: usize = 10;
/// Unit Data Command full message length.
pub const UNIT_DATA_COMMAND: usize = 6;
/// Unit Data Response full message length.
pub const UNIT_DATA_RESPONSE: usize = 18;
/// Channel Value Data Command full message length.
pub const CHANNEL_VALUE_DATA_COMMAND: usize = 6;
/// Channel Value Data Response full message length.
///
/// Because channel value data messages have variable response lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const CHANNEL_VALUE_DATA_RESPONSE: usize = MAX_MESSAGE;
/// Setup Request Command full message length.
pub const SETUP_REQUEST_COMMAND: usize = 6;
/// Setup Request Response full message length.
///
/// Because setup request messages have variable response lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const SETUP_REQUEST_RESPONSE: usize = MAX_MESSAGE;
/// LastRejectCode Command full message length.
pub const LAST_REJECT_CODE_COMMAND: usize = 6;
/// LastRejectCode Response full message length.
pub const LAST_REJECT_CODE_RESPONSE: usize = 7;
/// ConfigureBezel Command full message length.
pub const CONFIGURE_BEZEL_COMMAND: usize = 10;
/// ConfigureBezel Response full message length.
pub const CONFIGURE_BEZEL_RESPONSE: usize = 6;
/// PollWithAck Command full message length.
pub const POLL_WITH_ACK_COMMAND: usize = 6;
/// PollWithAck Response maximum full message length.
///
/// Because poll messages have variable response lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const POLL_WITH_ACK_RESPONSE: usize = MAX_MESSAGE;
/// EventAck Command full message length.
pub const EVENT_ACK_COMMAND: usize = 6;
/// EventAck Response full message length.
pub const EVENT_ACK_RESPONSE: usize = 6;
/// SetEncryptionKey Command full message length.
pub const SET_ENCRYPTION_KEY_COMMAND: usize = 14;
/// SetEncryptionKey Response full message length.
pub const SET_ENCRYPTION_KEY_RESPONSE: usize = 6;
/// SetGenerator Command full message length.
pub const SET_GENERATOR_COMMAND: usize = 14;
/// SetGenerator Response full message length.
pub const SET_GENERATOR_RESPONSE: usize = 6;
/// SetModulus Command full message length.
pub const SET_MODULUS_COMMAND: usize = 14;
/// SetModulus Response full message length.
pub const SET_MODULUS_RESPONSE: usize = 6;
/// RequestKeyExchange Command full message length.
pub const REQUEST_KEY_EXCHANGE_COMMAND: usize = 14;
/// RequestKeyExchange Response full message length.
pub const REQUEST_KEY_EXCHANGE_RESPONSE: usize = 14;
/// PayoutByDenomination Command maximum full message length.
///
/// Because payout messages have variable lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const PAYOUT_BY_DENOMINATION_COMMAND: usize = MAX_MESSAGE;
/// PayoutByDenomination Response full message length.
pub const PAYOUT_BY_DENOMINATION_RESPONSE: usize = 6;
/// Length of a serialized [PayoutDenomination](crate::PayoutDenomination).
pub const PAYOUT_BLOCK: usize = 9;
/// EnablePayout Command full message length.
pub const ENABLE_PAYOUT_COMMAND: usize = 7;
/// EnablePayout Response full message length.
pub const ENABLE_PAYOUT_RESPONSE: usize = 6;
/// DisablePayout Command full message length.
pub const DISABLE_PAYOUT_COMMAND: usize = 6;
/// DisablePayout Response full message length.
pub const DISABLE_PAYOUT_RESPONSE: usize = 6;
/// ProgramFirmware Command full message length.
pub const PROGRAM_FIRMWARE_COMMAND: usize = 7;
/// ProgramFirmware Response full message length.
pub const PROGRAM_FIRMWARE_RESPONSE: usize = 8;
/// FirmwareHeader Command full message length.
pub const FIRMWARE_HEADER_COMMAND: usize = 133;
/// FirmwareHeader Response full message length.
pub const FIRMWARE_HEADER_RESPONSE: usize = 6;
/// DownloadDataPacket Command full message length.
pub const DOWNLOAD_DATA_PACKET_COMMAND: usize = 139;
/// DownloadDataPacket Response full message length.
pub const DOWNLOAD_DATA_PACKET_RESPONSE: usize = 6;
/// Maximum size of a download data packet "line".
pub const DATA_PACKET_LINE: usize = 128;
/// Encrypted Command full message length.
///
/// Because encrypted messages have variable lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const ENCRYPTED_COMMAND: usize = MAX_ENCRYPTED_MESSAGE;
/// Encrypted Response full message length.
///
/// Because encrypted messages have variable lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const ENCRYPTED_RESPONSE: usize = MAX_ENCRYPTED_MESSAGE;
/// Wrapped Encrypted Message full message length.
///
/// Because encrypted messages have variable lengths, set the static length to maximum
/// possible. Actual length is determined by reading the LEN field.
pub const WRAPPED_ENCRYPTED_MESSAGE: usize = MAX_MESSAGE;
/// Header: `STX/STEX/STEXN | SEQID | LEN`
pub const HEADER: usize = 3;
/// Footer: CRC-16
pub const FOOTER: usize = 2;
/// Number of bytes in the header and footer of messages:
///
/// ```no_build,no_run
/// STX/STEX/STEXN | SEQID | LEN | ... | CRC_L | CRC_H
/// ```
pub const METADATA: usize = HEADER + FOOTER;
/// Length of metadata in encrypted messages.
///
/// ```no_build,no_run
/// STEX(1) | LEN(1) | COUNT(4) | ... | CRC_L(1) | CRC_H(1)
/// ```
pub const ENCRYPTED_METADATA: usize = 8;
/// Maximum data length for a message.
pub const MAX_DATA: usize = 255;
/// Maximum data length for an encrypted message.
///
/// Because encrypted messages must be wrapped in a standard SSP message, the full message must be
/// able to fit inside a standard SSP data field.
pub const MAX_ENCRYPTED_DATA: usize = MAX_DATA - ENCRYPTED_METADATA;
/// Maximum full length for a message.
pub const MAX_MESSAGE: usize = METADATA + MAX_DATA;
/// Maximum full length for an encrypted message.
pub const MAX_ENCRYPTED_MESSAGE: usize = MAX_DATA;

/// 24-bit number length in bytes.
pub const U24: usize = 3;
/// Length of [FirmwareVersion](crate::FirmwareVersion) data field.
pub const FIRMWARE_VERSION: usize = 4;
/// The size of an AES-128 block.
pub const AES: usize = 16;
/// The size of an AES-128 key.
pub const AES_KEY: usize = 16;

/// Gets the length of additional data needed to be a multiple of the AES block length.
pub fn aes_packing_len(raw_len: usize) -> usize {
    let rem = raw_len % AES;

    if rem == 0 {
        0
    } else {
        AES - rem
    }
}
