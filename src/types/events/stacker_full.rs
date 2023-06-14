use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [StackerFull](crate::ResponseStatus::StackerFull) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackerFullEvent;

impl StackerFullEvent {
    /// Creates a new [StackerFullEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for StackerFullEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::StackerFull => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::StackerFull))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for StackerFullEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for StackerFullEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&StackerFullEvent> for Event {
    fn from(_val: &StackerFullEvent) -> Self {
        Self::new("reject", &[]).unwrap()
    }
}

impl From<StackerFullEvent> for Event {
    fn from(val: StackerFullEvent) -> Self {
        (&val).into()
    }
}
