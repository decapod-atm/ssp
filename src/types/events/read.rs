use crate::{channel_value, ChannelValue, Error, ResponseStatus, Result};

use super::Event;

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

    /// Gets the [ChannelValue].
    pub fn value(&self) -> &ChannelValue {
        &self.value
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

impl From<&ReadEvent> for Event {
    fn from(val: &ReadEvent) -> Self {
        // `unwrap` is guaranteed not panic because the data length is in a valid range.
        Self::new(
            "cash_insertion_event",
            val.value().as_inner().to_le_bytes().as_ref(),
        )
        .unwrap()
    }
}

impl From<ReadEvent> for Event {
    fn from(val: ReadEvent) -> Self {
        (&val).into()
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
