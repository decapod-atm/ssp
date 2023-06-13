use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [Disabled](crate::ResponseStatus::Disabled) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisabledEvent;

impl DisabledEvent {
    /// Creates a new [DisabledEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for DisabledEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::Disabled => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::Disabled))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for DisabledEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for DisabledEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&DisabledEvent> for Event {
    fn from(_val: &DisabledEvent) -> Self {
        Self::new("disabled", &[]).unwrap()
    }
}

impl From<DisabledEvent> for Event {
    fn from(val: DisabledEvent) -> Self {
        (&val).into()
    }
}
