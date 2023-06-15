use crate::{std::fmt, ChannelValue};

use super::{Method, CLOSE_BRACE, OPEN_BRACE};

/// Represents a [Stack](crate::Method::Stack) event.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StackEvent {
    value: ChannelValue,
}

impl StackEvent {
    /// Creates a new [StackEvent].
    pub const fn new(value: ChannelValue) -> Self {
        Self { value }
    }

    /// Gets the [Method] for the [StackEvent].
    pub const fn method() -> Method {
        Method::Stack
    }

    /// Converts the [StackEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the [ChannelValue].
    pub const fn value(&self) -> ChannelValue {
        self.value
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        2
    }
}

impl From<&StackEvent> for &'static str {
    fn from(val: &StackEvent) -> Self {
        val.to_str()
    }
}

impl From<StackEvent> for &'static str {
    fn from(val: StackEvent) -> Self {
        (&val).into()
    }
}

impl From<ChannelValue> for StackEvent {
    fn from(val: ChannelValue) -> Self {
        Self::new(val)
    }
}

impl From<&ChannelValue> for StackEvent {
    fn from(val: &ChannelValue) -> Self {
        (*val).into()
    }
}

impl fmt::Display for StackEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = (OPEN_BRACE, CLOSE_BRACE);

        let method = self.to_str();
        let value = self.value();

        write!(f, "{o}\"{method}\": {o}\"value\": {value}{c}{c}")
    }
}
