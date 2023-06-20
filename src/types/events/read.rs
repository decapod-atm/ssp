use crate::{channel_value, std::fmt, ChannelValue, Error, ResponseStatus, Result};

use super::{Method, CLOSE_BRACE, OPEN_BRACE};

/// Represents a [Read](crate::ResponseStatus::Read) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReadEvent {
    value: ChannelValue,
}

impl ReadEvent {
    /// Creates a new [ReadEvent] from the [ChannelValue].
    pub const fn new(value: ChannelValue) -> Self {
        Self { value }
    }

    /// Gets the [Method] for the [ReadEvent].
    pub const fn method() -> Method {
        Method::Read
    }

    /// Converts the [ReadEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the [ChannelValue].
    pub const fn value(&self) -> ChannelValue {
        self.value
    }

    /// Sets the [ChannelValue].
    pub fn set_value(&mut self, value: ChannelValue) {
        self.value = value;
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        2
    }
}

impl TryFrom<&[u8]> for ReadEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            0..=1 => Err(Error::InvalidLength((val.len(), 2))),
            _ => {
                let event = ResponseStatus::from(val[0]);
                if event == ResponseStatus::Read {
                    Ok(Self::new(channel_value(val[1] as usize)?))
                } else {
                    Err(Error::InvalidEvent((event, ResponseStatus::Read)))
                }
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for ReadEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for ReadEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<ChannelValue> for ReadEvent {
    fn from(val: ChannelValue) -> Self {
        Self::new(val)
    }
}

impl From<&ChannelValue> for ReadEvent {
    fn from(val: &ChannelValue) -> Self {
        (*val).into()
    }
}

impl fmt::Display for ReadEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = (OPEN_BRACE, CLOSE_BRACE);

        let method = self.to_str();
        let value = self.value();

        write!(f, "{o}\"{method}\": {o}\"value\": {value}{c}{c}",)
    }
}

impl Default for ReadEvent {
    fn default() -> Self {
        Self::new(ChannelValue::default())
    }
}
