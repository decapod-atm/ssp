use alloc::string::String;

use crate::std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};

use crate::{
    impl_default, std::fmt, ResponseOps, SetupRequestResponse, UnitDataResponse, CLOSE_BRACE,
    OPEN_BRACE,
};

use super::{
    CountryCode, FirmwareVersion, ProtocolVersion, ResponseStatus, UnitType, ValueMultiplier,
};

/// Represents the status of the device.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceStatus {
    status: ResponseStatus,
    unit_type: UnitType,
    firmware_version: FirmwareVersion,
    country_code: CountryCode,
    value_multiplier: ValueMultiplier,
    protocol_version: ProtocolVersion,
    dataset_version: String,
    cashbox_attached: bool,
}

impl DeviceStatus {
    /// Creates a new [DeviceStatus].
    pub const fn new() -> Self {
        Self {
            status: ResponseStatus::Ok,
            unit_type: UnitType::from_inner(0),
            firmware_version: FirmwareVersion::from_inner(0),
            country_code: CountryCode::from_inner(0),
            value_multiplier: ValueMultiplier::from_inner(0),
            protocol_version: ProtocolVersion::Reserved,
            dataset_version: String::new(),
            cashbox_attached: false,
        }
    }

    /// Gets the [ResponseStatus].
    pub const fn response_status(&self) -> ResponseStatus {
        self.status
    }

    pub fn set_response_status(&mut self, status: ResponseStatus) {
        self.status = status;
    }

    /// Builder function that sets the [ResponseStatus].
    pub fn with_response_status(mut self, status: ResponseStatus) -> Self {
        self.set_response_status(status);
        self
    }

    /// Gets the [UnitType].
    pub const fn unit_type(&self) -> UnitType {
        self.unit_type
    }

    /// Sets the [UnitType].
    pub fn set_unit_type(&mut self, unit_type: UnitType) {
        self.unit_type = unit_type;
    }

    /// Builder function that sets the [UnitType].
    pub fn with_unit_type(mut self, unit_type: UnitType) -> Self {
        self.set_unit_type(unit_type);
        self
    }

    /// Gets the [FirmwareVersion].
    pub const fn firmware_version(&self) -> FirmwareVersion {
        self.firmware_version
    }

    /// Sets the [FirmwareVersion].
    pub fn set_firmware_version(&mut self, firmware_version: FirmwareVersion) {
        self.firmware_version = firmware_version;
    }

    /// Builder function that sets the [FirmwareVersion].
    pub fn with_firmware_version(mut self, firmware_version: FirmwareVersion) -> Self {
        self.set_firmware_version(firmware_version);
        self
    }

    /// Gets the [CountryCode].
    pub const fn country_code(&self) -> CountryCode {
        self.country_code
    }

    /// Sets the [CountryCode].
    pub fn set_country_code(&mut self, country_code: CountryCode) {
        self.country_code = country_code;
    }

    /// Builder function that sets the [CountryCode].
    pub fn with_country_code(mut self, country_code: CountryCode) -> Self {
        self.set_country_code(country_code);
        self
    }

    /// Gets the [ValueMultiplier].
    pub const fn value_multiplier(&self) -> ValueMultiplier {
        self.value_multiplier
    }

    /// Sets the [ValueMultiplier].
    pub fn set_value_multiplier(&mut self, value_multiplier: ValueMultiplier) {
        self.value_multiplier = value_multiplier
    }

    /// Builder function that sets the [ValueMultiplier].
    pub fn with_value_multiplier(mut self, value_multiplier: ValueMultiplier) -> Self {
        self.set_value_multiplier(value_multiplier);
        self
    }

    /// Gets the [ProtocolVersion].
    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }

    /// Sets the [ProtocolVersion].
    pub fn set_protocol_version(&mut self, protocol_version: ProtocolVersion) {
        self.protocol_version = protocol_version;
    }

    /// Builder function that sets the [ProtocolVersion].
    pub fn with_protocol_version(mut self, protocol_version: ProtocolVersion) -> Self {
        self.set_protocol_version(protocol_version);
        self
    }

    /// Gets the dataset version.
    pub fn dataset_version(&self) -> &str {
        self.dataset_version.as_str()
    }

    /// Sets the dataset version.
    pub fn set_dataset_version(&mut self, dataset_version: &str) {
        self.dataset_version = dataset_version.into();
    }

    /// Builder function that sets the dataset version.
    pub fn with_dataset_version(mut self, dataset_version: &str) -> Self {
        self.set_dataset_version(dataset_version);
        self
    }

    /// Gets whether the cashbox is attached.
    pub const fn cashbox_attached(&self) -> bool {
        self.cashbox_attached
    }

    /// Sets whether the cashbox is attached.
    pub fn set_cashbox_attached(&mut self, cashbox_attached: bool) {
        self.cashbox_attached = cashbox_attached;
    }

    /// Builder function that sets whether the cashbox is attached.
    pub fn with_cashbox_attached(mut self, cashbox_attached: bool) -> Self {
        self.set_cashbox_attached(cashbox_attached);
        self
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
            dataset_version: String::new(),
            cashbox_attached: false,
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
            dataset_version: String::new(),
            cashbox_attached: false,
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
        let dataset = self.dataset_version();
        let cashbox = self.cashbox_attached();

        write!(f, "{o}\"response_status\": \"{status}\", \"unit_type\": {unit_type}, \"firmware_version\": \"{firmware}\", \"country_code\": \"{country_code}\", \"value_multiplier\": {vm}, \"protocol_version\": \"{protocol}\", \"dataset_version\": \"{dataset}\", \"cashbox_attached\": {cashbox}{c}")
    }
}

/// Atomic integer representation of the [DeviceStatus].
///
/// Useful for instantiating a safe, static global state variable.
#[repr(C)]
#[derive(Debug)]
pub struct AtomicDeviceStatus {
    status: AtomicU8,
    unit_type: AtomicU8,
    firmware_version: AtomicU32,
    country_code: AtomicU32,
    value_multiplier: AtomicU32,
    protocol_version: AtomicU8,
    cashbox_attached: AtomicBool,
}

impl AtomicDeviceStatus {
    /// Creates a new [AtomicDeviceStatus].
    pub const fn new() -> Self {
        Self {
            status: AtomicU8::new(ResponseStatus::Ok.to_u8()),
            unit_type: AtomicU8::new(0),
            firmware_version: AtomicU32::new(0),
            country_code: AtomicU32::new(0),
            value_multiplier: AtomicU32::new(0),
            protocol_version: AtomicU8::new(0),
            cashbox_attached: AtomicBool::new(false),
        }
    }

    /// Gets the [ResponseStatus].
    pub fn response_status(&self) -> ResponseStatus {
        ResponseStatus::from_u8(self.status.load(Ordering::Relaxed))
    }

    /// Sets the [ResponseStatus].
    pub fn set_response_status(&self, status: ResponseStatus) {
        self.status.store(status.to_u8(), Ordering::SeqCst);
    }

    /// Builder function that sets the [ResponseStatus].
    pub fn with_response_status(self, status: ResponseStatus) -> Self {
        self.set_response_status(status);
        self
    }

    /// Gets the [UnitType].
    pub fn unit_type(&self) -> UnitType {
        UnitType::from_inner(self.unit_type.load(Ordering::Relaxed))
    }

    /// Sets the [UnitType].
    pub fn set_unit_type(&self, unit_type: UnitType) {
        self.unit_type.store(unit_type.as_inner(), Ordering::SeqCst);
    }

    /// Builder function that sets the [UnitType].
    pub fn with_unit_type(self, unit_type: UnitType) -> Self {
        self.set_unit_type(unit_type);
        self
    }

    /// Gets the [FirmwareVersion].
    pub fn firmware_version(&self) -> FirmwareVersion {
        FirmwareVersion::from_inner(self.firmware_version.load(Ordering::Relaxed))
    }

    /// Sets the [FirmwareVersion].
    pub fn set_firmware_version(&self, firmware_version: FirmwareVersion) {
        self.firmware_version
            .store(firmware_version.as_inner(), Ordering::SeqCst);
    }

    /// Builder function that sets the [FirmwareVersion].
    pub fn with_firmware_version(self, firmware_version: FirmwareVersion) -> Self {
        self.set_firmware_version(firmware_version);
        self
    }

    /// Gets the [CountryCode].
    pub fn country_code(&self) -> CountryCode {
        CountryCode::from_inner(self.country_code.load(Ordering::Relaxed))
    }

    /// Sets the [CountryCode].
    pub fn set_country_code(&self, country_code: CountryCode) {
        self.country_code
            .store(country_code.as_inner(), Ordering::SeqCst);
    }

    /// Builder function that sets the [CountryCode].
    pub fn with_country_code(self, country_code: CountryCode) -> Self {
        self.set_country_code(country_code);
        self
    }

    /// Gets the [ValueMultiplier].
    pub fn value_multiplier(&self) -> ValueMultiplier {
        ValueMultiplier::from_inner(self.value_multiplier.load(Ordering::Relaxed))
    }

    /// Sets the [ValueMultiplier].
    pub fn set_value_multiplier(&self, value_multiplier: ValueMultiplier) {
        self.value_multiplier
            .store(value_multiplier.as_inner(), Ordering::SeqCst);
    }

    /// Builder function that sets the [ValueMultiplier].
    pub fn with_value_multiplier(self, value_multiplier: ValueMultiplier) -> Self {
        self.set_value_multiplier(value_multiplier);
        self
    }

    /// Gets the [ProtocolVersion].
    pub fn protocol_version(&self) -> ProtocolVersion {
        ProtocolVersion::from_u8(self.protocol_version.load(Ordering::Relaxed))
    }

    /// Sets the [ProtocolVersion].
    pub fn set_protocol_version(&self, protocol_version: ProtocolVersion) {
        self.protocol_version
            .store(protocol_version.to_u8(), Ordering::SeqCst);
    }

    /// Builder function that sets the [ProtocolVersion].
    pub fn with_protocol_version(self, protocol_version: ProtocolVersion) -> Self {
        self.set_protocol_version(protocol_version);
        self
    }

    /// Gets whether the cashbox is attached.
    pub fn cashbox_attached(&self) -> bool {
        self.cashbox_attached.load(Ordering::Relaxed)
    }

    /// Sets whether the cashbox is attached.
    pub fn set_cashbox_attached(&self, cashbox_attached: bool) {
        self.cashbox_attached
            .store(cashbox_attached, Ordering::SeqCst);
    }

    /// Builder function that sets whether the cashbox is attached.
    pub fn with_cashbox_attached(self, cashbox_attached: bool) -> Self {
        self.set_cashbox_attached(cashbox_attached);
        self
    }
}

impl From<&DeviceStatus> for AtomicDeviceStatus {
    fn from(val: &DeviceStatus) -> Self {
        Self::new()
            .with_response_status(val.response_status())
            .with_unit_type(val.unit_type())
            .with_firmware_version(val.firmware_version())
            .with_country_code(val.country_code())
            .with_value_multiplier(val.value_multiplier())
            .with_protocol_version(val.protocol_version())
            .with_cashbox_attached(val.cashbox_attached())
    }
}

impl From<DeviceStatus> for AtomicDeviceStatus {
    fn from(val: DeviceStatus) -> Self {
        (&val).into()
    }
}

impl From<&AtomicDeviceStatus> for DeviceStatus {
    fn from(val: &AtomicDeviceStatus) -> Self {
        Self::new()
            .with_response_status(val.response_status())
            .with_unit_type(val.unit_type())
            .with_firmware_version(val.firmware_version())
            .with_country_code(val.country_code())
            .with_value_multiplier(val.value_multiplier())
            .with_protocol_version(val.protocol_version())
            .with_cashbox_attached(val.cashbox_attached())
    }
}

impl From<AtomicDeviceStatus> for DeviceStatus {
    fn from(val: AtomicDeviceStatus) -> Self {
        (&val).into()
    }
}

impl_default!(AtomicDeviceStatus);
