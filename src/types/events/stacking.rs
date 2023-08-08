use crate::{impl_default, std::fmt, Error, ResponseStatus, Result};

use super::Method;

/// Represents a [Stacking](crate::ResponseStatus::Stacking) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackingEvent;

impl StackingEvent {
    /// Creates a new [StackingEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [Method] for the [StackingEvent].
    pub const fn method() -> Method {
        Method::Stacking
    }

    /// Converts the [StackingEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
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

impl From<&StackingEvent> for &'static str {
    fn from(val: &StackingEvent) -> Self {
        val.to_str()
    }
}

impl From<StackingEvent> for &'static str {
    fn from(val: StackingEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for StackingEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(StackingEvent);
