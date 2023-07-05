use crate::{std::fmt, DeviceStatus};

use super::{Method, CLOSE_BRACE, OPEN_BRACE};

/// Represents a [Status](crate::ResponseStatus::Status) event.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatusEvent {
    details: DeviceStatus,
}

impl StatusEvent {
    /// Creates a new [StatusEvent].
    pub const fn new(details: DeviceStatus) -> Self {
        Self { details }
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
        &self.details
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "jsonrpc")]
    use super::*;
    #[cfg(feature = "jsonrpc")]
    use crate::Result;

    #[cfg(feature = "jsonrpc")]
    #[test]
    fn test_status_event_serde() -> Result<()> {
        let event = StatusEvent::default();
        let exp_event_str = "{\"details\":{\"status\":\"Ok\",\"unit_type\":0,\"firmware_version\":0,\"country_code\":0,\"value_multiplier\":0,\"protocol_version\":\"Reserved\",\"cashbox_attached\":false}}";

        assert_eq!(serde_json::to_string(&event)?.as_str(), exp_event_str);

        Ok(())
    }
}
