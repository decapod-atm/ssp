use crate::{impl_default, std::fmt, Error, ResponseStatus, Result};

use super::Method;

/// Represents a [Reset](crate::ResponseStatus::Reset) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResetEvent;

impl ResetEvent {
    /// Creates a new [ResetEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [Method] for the [ResetEvent].
    pub const fn method() -> Method {
        Method::Reset
    }

    /// Converts the [ResetEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
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

impl From<&ResetEvent> for &'static str {
    fn from(val: &ResetEvent) -> Self {
        val.to_str()
    }
}

impl From<ResetEvent> for &'static str {
    fn from(val: ResetEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for ResetEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(ResetEvent);
