use crate::{
    impl_default, std::fmt, ResponseOps, SetupRequestResponse, UnitDataResponse, CLOSE_BRACE,
    OPEN_BRACE,
};

use super::{
    CountryCode, FirmwareVersion, ProtocolVersion, ResponseStatus, UnitType, ValueMultiplier,
};

/// Represents the status of the device.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceStatus {
    status: ResponseStatus,
    unit_type: UnitType,
    firmware_version: FirmwareVersion,
    country_code: CountryCode,
    value_multiplier: ValueMultiplier,
    protocol_version: ProtocolVersion,
}

impl DeviceStatus {
    pub const fn new() -> Self {
        Self {
            status: ResponseStatus::Ok,
            unit_type: UnitType::from_inner(0),
            firmware_version: FirmwareVersion::from_inner(0),
            country_code: CountryCode::from_inner(0),
            value_multiplier: ValueMultiplier::from_inner(0),
            protocol_version: ProtocolVersion::Reserved,
        }
    }

    /// Gets the [ResponseStatus].
    pub const fn response_status(&self) -> ResponseStatus {
        self.status
    }

    /// Gets the [UnitType].
    pub const fn unit_type(&self) -> UnitType {
        self.unit_type
    }

    /// Gets the [FirmwareVersion].
    pub const fn firmware_version(&self) -> FirmwareVersion {
        self.firmware_version
    }

    /// Gets the [CountryCode].
    pub const fn country_code(&self) -> CountryCode {
        self.country_code
    }

    /// Gets the [ValueMultiplier].
    pub const fn value_multiplier(&self) -> ValueMultiplier {
        self.value_multiplier
    }

    /// Gets the [ProtocolVersion].
    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
}

impl From<&SetupRequestResponse> for DeviceStatus {
    fn from(val: &SetupRequestResponse) -> Self {
        Self {
            status: val.response_status(),
            unit_type: val.unit_type(),
            firmware_version: val.firmware_version(),
            country_code: val.country_code(),
            value_multiplier: val.value_multiplier(),
            protocol_version: val.protocol_version().unwrap_or(ProtocolVersion::Reserved),
        }
    }
}

impl From<SetupRequestResponse> for DeviceStatus {
    fn from(val: SetupRequestResponse) -> Self {
        (&val).into()
    }
}

impl From<&UnitDataResponse> for DeviceStatus {
    fn from(val: &UnitDataResponse) -> Self {
        Self {
            status: val.response_status(),
            unit_type: val.unit_type(),
            firmware_version: val.firmware_version(),
            country_code: val.country_code(),
            value_multiplier: val.value_multiplier(),
            protocol_version: val.protocol_version(),
        }
    }
}

impl From<UnitDataResponse> for DeviceStatus {
    fn from(val: UnitDataResponse) -> Self {
        (&val).into()
    }
}

impl_default!(DeviceStatus);

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (o, c) = (OPEN_BRACE, CLOSE_BRACE);
        let status = self.response_status();
        let unit_type = self.unit_type();
        let firmware = self.firmware_version();
        let country_code = self.country_code();
        let vm = self.value_multiplier();
        let protocol = self.protocol_version();

        write!(f, "{o}\"response_status\": \"{status}\", \"unit_type\": {unit_type}, \"firmware_version\": \"{firmware}\", \"country_code\": \"{country_code}\", \"value_multiplier\": {vm}, \"protocol_version\": \"{protocol}\"{c}")
    }
}
