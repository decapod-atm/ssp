use crate::{impl_default, std::fmt};

use super::Method;

/// Represents a [Reject](crate::Method::Reject) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RejectEvent;

impl RejectEvent {
    /// Creates a new [RejectEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [RejectEvent] for the [RejectEvent].
    pub const fn method() -> Method {
        Method::Reject
    }

    /// Converts the [RejectEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl From<&RejectEvent> for &'static str {
    fn from(val: &RejectEvent) -> Self {
        val.to_str()
    }
}

impl From<RejectEvent> for &'static str {
    fn from(val: RejectEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for RejectEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(RejectEvent);
