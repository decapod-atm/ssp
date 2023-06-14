use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [Rejected](crate::ResponseStatus::Rejected) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RejectedEvent;

impl RejectedEvent {
    /// Creates a new [RejectedEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for RejectedEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::Rejected => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::Rejected))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for RejectedEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for RejectedEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&RejectedEvent> for Event {
    fn from(_val: &RejectedEvent) -> Self {
        Self::new("reject", &[]).unwrap()
    }
}

impl From<RejectedEvent> for Event {
    fn from(val: RejectedEvent) -> Self {
        (&val).into()
    }
}
