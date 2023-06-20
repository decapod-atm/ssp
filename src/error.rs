//! Error types for the library.

use crate::std::{self, fmt};

use alloc::{format, string::String};

use crate::{MessageType, ResponseStatus, STX};

/// Result wrapper type for the library.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for the library.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Error {
    Generic(i64),
    Crc((u16, u16)),
    Encryption(ResponseStatus),
    InvalidBarcodeCharacters(u8),
    InvalidDataLength((usize, usize)),
    InvalidInhibitChannels,
    InvalidLength((usize, usize)),
    InvalidEvent((ResponseStatus, ResponseStatus)),
    InvalidMessage(MessageType),
    InvalidMessageRaw((MessageType, u8)),
    InvalidStatus((ResponseStatus, ResponseStatus)),
    InvalidSTX(u8),
    PollingReinit,
    QueueTimeout,
    #[cfg(feature = "std")]
    Io(String),
    #[cfg(feature = "std")]
    SerialPort(String),
    Utf8(String),
    Status(ResponseStatus),
    Timeout(String),
    #[cfg(feature = "jsonrpc")]
    JsonRpc(String),
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
            Error::InvalidEvent((have, exp)) => {
                write!(f, "Invalid device event, have: {have}, expected: {exp}")
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
            Error::InvalidStatus((have, exp)) => {
                write!(f, "Invalid response status, have: {have}, expected: {exp}")
            }
            Error::InvalidSTX(err) => {
                write!(f, "Invalid message STX, have: {err}, expected: {STX}")
            }
            Error::PollingReinit => {
                write!(f, "Background polling already initialized")
            }
            Error::QueueTimeout => {
                write!(f, "Failed to retrieve a queued event before timeout")
            }
            #[cfg(feature = "std")]
            Error::Io(err) => write!(f, "I/O error: {err}"),
            #[cfg(feature = "std")]
            Error::SerialPort(err) => write!(f, "Serial port communication error: {err:?}"),
            Error::Status(err) => write!(f, "Response status: {err}"),
            Error::Timeout(err) => write!(f, "Failed to perform action before timeout: {err}"),
            Error::Utf8(err) => write!(f, "UTF8 error occurred: {err}"),
            #[cfg(feature = "jsonrpc")]
            Error::JsonRpc(err) => write!(f, "Failed processing JSON-RPC message(s): {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Self {
        Self::SerialPort(format!("{err}"))
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(format!("{err}"))
    }
}

impl From<()> for Error {
    fn from(_err: ()) -> Self {
        Self::Generic(-1)
    }
}

#[cfg(feature = "jsonrpc")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonRpc(format!("{err}"))
    }
}

#[cfg(feature = "jsonrpc")]
impl From<smol_jsonrpc::Error> for Error {
    fn from(err: smol_jsonrpc::Error) -> Self {
        Self::JsonRpc(format!("{err}"))
    }
}

#[cfg(feature = "jsonrpc")]
impl From<&Error> for smol_jsonrpc::Error {
    fn from(err: &Error) -> Self {
        match err {
            Error::JsonRpc(e) => Self::new()
                .with_code(smol_jsonrpc::ErrorCode::ParseError)
                .with_message(e.as_str()),
            error => Self::new()
                .with_code(smol_jsonrpc::ErrorCode::InternalError)
                .with_message(format!("{error}").as_str()),
        }
    }
}

#[cfg(feature = "jsonrpc")]
impl From<Error> for smol_jsonrpc::Error {
    fn from(err: Error) -> Self {
        (&err).into()
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Utf8(format!("{err}"))
    }
}

#[cfg(feature = "std")]
impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::Io(format!("{err}"))
    }
}
