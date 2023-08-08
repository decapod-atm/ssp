use crate::{std::fmt, ProtocolVersion};

use super::Method;

/// Represents a [Enable](crate::ResponseEnable::Enable) event.
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnableEvent {
    protocol: ProtocolVersion,
}

impl EnableEvent {
    /// Creates a new [EnableEvent].
    pub const fn new(protocol: ProtocolVersion) -> Self {
        Self { protocol }
    }

    /// Gets the [Method] for the [EnableEvent].
    pub const fn method() -> Method {
        Method::Enable
    }

    /// Converts the [EnableEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets the [ProtocolVersion].
    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol
    }

    /// Sets the [ProtocolVersion].
    pub fn set_protocol_version(&mut self, protocol: ProtocolVersion) {
        self.protocol = protocol;
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        1
    }
}

impl From<&EnableEvent> for &'static str {
    fn from(val: &EnableEvent) -> Self {
        val.to_str()
    }
}

impl From<EnableEvent> for &'static str {
    fn from(val: EnableEvent) -> Self {
        (&val).into()
    }
}

impl From<ProtocolVersion> for EnableEvent {
    fn from(val: ProtocolVersion) -> Self {
        Self::new(val)
    }
}

impl From<&ProtocolVersion> for EnableEvent {
    fn from(val: &ProtocolVersion) -> Self {
        (*val).into()
    }
}

impl fmt::Display for EnableEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = self.to_str();
        let protocol = self.protocol_version();

        write!(f, r#"{{"{method}": {{"protocol_version": {protocol}}}"#)
    }
}
