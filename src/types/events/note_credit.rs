use crate::{channel_value, ChannelValue, Error, ResponseStatus, Result};

use super::Event;

/// Represents a [NoteCredit](crate::ResponseStatus::NoteCredit) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NoteCreditEvent {
    value: ChannelValue,
}

impl NoteCreditEvent {
    /// Creates a new [NoteCreditEvent] from the [ChannelValue].
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

impl TryFrom<&[u8]> for NoteCreditEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        match val.len() {
            0..=1 => Err(Error::InvalidLength((val.len(), 2))),
            _ => match ResponseStatus::from(val[0]) {
                ResponseStatus::NoteCredit => Ok(Self::new(channel_value(val[1] as usize)?)),
                event => Err(Error::InvalidEvent((event, ResponseStatus::NoteCredit))),
            },
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for NoteCreditEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for NoteCreditEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&NoteCreditEvent> for Event {
    fn from(val: &NoteCreditEvent) -> Self {
        // `unwrap` is guaranteed not panic because the data length is in a valid range.
        Self::new("stack", val.value().as_inner().to_le_bytes().as_ref()).unwrap()
    }
}

impl From<NoteCreditEvent> for Event {
    fn from(val: NoteCreditEvent) -> Self {
        (&val).into()
    }
}

impl From<ChannelValue> for NoteCreditEvent {
    fn from(val: ChannelValue) -> Self {
        Self::new(val)
    }
}

impl From<&ChannelValue> for NoteCreditEvent {
    fn from(val: &ChannelValue) -> Self {
        (*val).into()
    }
}
