use crate::{impl_default, std::fmt};

use super::Method;

/// Represents a [Disable](crate::ResponseDisable::Disable) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisableEvent;

impl DisableEvent {
    /// Creates a new [DisableEvent].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the [Method] for the [DisableEvent].
    pub const fn method() -> Method {
        Method::Disable
    }

    /// Converts the [DisableEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl From<&DisableEvent> for &'static str {
    fn from(val: &DisableEvent) -> Self {
        val.to_str()
    }
}

impl From<DisableEvent> for &'static str {
    fn from(val: DisableEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DisableEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(DisableEvent);
