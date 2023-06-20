use crate::{impl_default, std::fmt, Error, ResponseStatus, Result};

use super::{Method, CLOSE_BRACE, OPEN_BRACE};

/// Represents a [Rejecting](crate::ResponseStatus::Rejecting) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RejectingEvent;

impl RejectingEvent {
    /// Creates a new [RejectingEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [Method] for the [RejectingEvent].
    pub const fn method() -> Method {
        Method::Rejecting
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl TryFrom<&[u8]> for RejectingEvent {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        if val.is_empty() {
            Err(Error::InvalidLength((val.len(), 1)))
        } else {
            match ResponseStatus::from(val[0]) {
                ResponseStatus::Rejecting => Ok(Self::new()),
                event => Err(Error::InvalidEvent((event, ResponseStatus::Rejecting))),
            }
        }
    }
}

impl<const N: usize> TryFrom<[u8; N]> for RejectingEvent {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for RejectingEvent {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl fmt::Display for RejectingEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{OPEN_BRACE}\"{}\"{CLOSE_BRACE}",
            Self::method().to_str()
        )
    }
}

impl_default!(RejectingEvent);
