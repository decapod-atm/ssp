use crate::{impl_default, std::fmt, PayoutDenominationList};

use super::Method;

/// Represents a [Dispense](crate::PayoutDenominationCommand) event.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DispenseEvent(PayoutDenominationList);

impl DispenseEvent {
    /// Creates a new [DispenseEvent].
    pub const fn new() -> Self {
        Self(PayoutDenominationList::new())
    }

    /// Creates a new [DispenseEvent] with the provided parameter.
    pub const fn create(list: PayoutDenominationList) -> Self {
        Self(list)
    }

    /// Gets the [Method] for the [DispenseEvent].
    pub const fn method() -> Method {
        Method::Dispense
    }

    /// Converts the [DispenseEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }

    /// Gets a reference to the inner representation of the [DispenseEvent].
    pub const fn as_inner(&self) -> &PayoutDenominationList {
        &self.0
    }

    /// Gets a mutable reference to the inner representation of the [DispenseEvent].
    pub fn as_inner_mut(&mut self) -> &mut PayoutDenominationList {
        &mut self.0
    }

    /// Consumes the [DispenseEvent], returning its inner repreesentation.
    pub fn to_inner(self) -> PayoutDenominationList {
        self.0
    }
}

impl From<&DispenseEvent> for &'static str {
    fn from(val: &DispenseEvent) -> Self {
        val.to_str()
    }
}

impl From<DispenseEvent> for &'static str {
    fn from(val: DispenseEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for DispenseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"{}"}}"#, self.to_str())
    }
}

impl_default!(DispenseEvent);
