//! Error types for the library.

use crate::std;
use std::fmt;

use crate::{MessageType, ResponseStatus, STX};

/// Result wrapper type for the library.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for the library.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    Generic(i64),
    Crc((u16, u16)),
    Encryption(ResponseStatus),
    InvalidBarcodeCharacters(u8),
    InvalidDataLength((usize, usize)),
    InvalidInhibitChannels,
    InvalidLength((usize, usize)),
    InvalidMessage(MessageType),
    InvalidMessageRaw((MessageType, u8)),
    InvalidSTX(u8),
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
    #[cfg(feature = "std")]
    SerialPort(serialport::ErrorKind),
    Status(ResponseStatus),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Crc((have, exp)) => {
                write!(f, "Bad CRC-16, have: 0x{have:04x}, expected: 0x{exp:04x}")
            }
            Error::Encryption(err) => write!(f, "Error sending encrypted message: {err}"),
            Error::Generic(err) => write!(f, "Generic: {err}"),
            Error::InvalidBarcodeCharacters(num) => {
                let min = crate::BARCODE_MIN_CHARS;
                let max = crate::BARCODE_MAX_CHARS;

                write!(
                    f,
                    "Invalid number of barcode characters: {num}, min: {min}, max: {max}"
                )
            }
            Error::InvalidDataLength((have, exp)) => {
                write!(f, "Invalid data length, have: {have}, expected: {exp}")
            }
            Error::InvalidInhibitChannels => {
                write!(f, "Trying to set an invalid number of inhibit channels")
            }
            Error::InvalidLength((have, exp)) => {
                write!(f, "Invalid message length, have: {have}, expected: {exp}")
            }
            Error::InvalidMessage(err) => write!(f, "Invalid message type: {err}"),
            Error::InvalidMessageRaw((msg, raw)) => {
                write!(f, "Invalid message type: {msg}, raw type: 0x{raw:02x}")
            }
            Error::InvalidSTX(err) => {
                write!(f, "Invalid message STX, have: {err}, expected: {STX}")
            }
            #[cfg(feature = "std")]
            Error::Io(err) => write!(f, "I/O error: {err}"),
            #[cfg(feature = "std")]
            Error::SerialPort(err) => write!(f, "Serial port communication error: {err:?}"),
            Error::Status(err) => write!(f, "Response status: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Self {
        Self::SerialPort(err.kind())
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.kind())
    }
}
