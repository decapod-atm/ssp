use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [UnsafeJam](crate::ResponseStatus::UnsafeJam) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnsafeJamEvent;

impl UnsafeJamEvent {
    /// Creates a new [UnsafeJamEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for UnsafeJamEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::UnsafeJam => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::UnsafeJam))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for UnsafeJamEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for UnsafeJamEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&UnsafeJamEvent> for Event {
    fn from(_val: &UnsafeJamEvent) -> Self {
        Self::new("jam", &[]).unwrap()
    }
}

impl From<UnsafeJamEvent> for Event {
    fn from(val: UnsafeJamEvent) -> Self {
        (&val).into()
    }
}
