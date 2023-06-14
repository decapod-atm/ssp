use crate::{Error, ResponseStatus, Result};

use super::Event;

/// Represents a [Stacking](crate::ResponseStatus::Stacking) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackingEvent;

impl StackingEvent {
    /// Creates a new [StackingEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for StackingEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::Stacking => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::Stacking))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for StackingEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for StackingEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl From<&StackingEvent> for Event {
    fn from(_val: &StackingEvent) -> Self {
        Self::new("stacking", &[]).unwrap()
    }
}

impl From<StackingEvent> for Event {
    fn from(val: StackingEvent) -> Self {
        (&val).into()
    }
}
