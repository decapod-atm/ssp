use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [Reset](crate::ResponseStatus::Reset) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResetEvent;

impl ResetEvent {
    /// Creates a new [ResetEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for ResetEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::DeviceReset => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::DeviceReset))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for ResetEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for ResetEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&ResetEvent> for Event {
    fn from(_val: &ResetEvent) -> Self {
        Self::new("reset", &[]).unwrap()
    }
}

impl From<ResetEvent> for Event {
    fn from(val: ResetEvent) -> Self {
        (&val).into()
    }
}
