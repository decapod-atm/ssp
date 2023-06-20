use crate::{std::fmt, DeviceStatus};

use super::{Method, CLOSE_BRACE, OPEN_BRACE};

/// Represents a [Status](crate::ResponseStatus::Status) event.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatusEvent {
    status: DeviceStatus,
}

impl StatusEvent {
    /// Creates a new [StatusEvent].
    pub const fn new(status: DeviceStatus) -> Self {
        Self { status }
    }

    /// Gets the [Method] for the [StatusEvent].
    pub const fn method() -> Method {
        Method::Status
    }

    /// Converts the [StatusEvent] to a string.
    pub const fn to_str(&self) -> &'static str {
        Self::method().to_str()
    }

    /// Gets a reference to the [DeviceStatus].
    pub const fn device_status(&self) -> &DeviceStatus {
        &self.status
    }

    /// Gets the length of the event in a [PollResponse](crate::PollResponse).
    pub const fn len() -> usize {
        14
    }
}

impl From<&StatusEvent> for &'static str {
    fn from(val: &StatusEvent) -> Self {
        val.to_str()
    }
}

impl From<StatusEvent> for &'static str {
    fn from(val: StatusEvent) -> Self {
        (&val).into()
    }
}

impl fmt::Display for StatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = (OPEN_BRACE, CLOSE_BRACE);
        let method = self.to_str();
        let status = self.device_status();
        write!(f, "{o}\"{method}\": {status}{c}")
    }
}

impl Default for StatusEvent {
    fn default() -> Self {
        Self::new(DeviceStatus::default())
    }
}
