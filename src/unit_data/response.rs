use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_ops, len, std::fmt,
    CountryCode, FirmwareVersion, MessageOps, MessageType, ProtocolVersion, ResponseOps, UnitType,
    ValueMultiplier,
};

mod index {
    use crate::len;

    pub const UNIT_TYPE: usize = 4;
    pub const FIRMWARE_VERSION: usize = UNIT_TYPE + 1;
    pub const FIRMWARE_VERSION_END: usize = FIRMWARE_VERSION + len::FIRMWARE_VERSION;
    pub const COUNTRY_CODE: usize = FIRMWARE_VERSION_END;
    pub const COUNTRY_CODE_END: usize = COUNTRY_CODE + len::U24;
    pub const VALUE_MULTIPLIER: usize = COUNTRY_CODE_END;
    pub const VALUE_MULTIPLIER_END: usize = VALUE_MULTIPLIER + len::U24;
    pub const PROTOCOL_VERSION: usize = VALUE_MULTIPLIER_END;
}

/// UnitData - Response (0x0A)
///
/// Represents a response to an [UnitDataCommand](crate::UnitDataCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitDataResponse {
    buf: [u8; len::UNIT_DATA_RESPONSE],
}

impl UnitDataResponse {
    /// Creates a new [UnitDataResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::UNIT_DATA_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the [UnitType].
    pub fn unit_type(&self) -> UnitType {
        self.buf[index::UNIT_TYPE].into()
    }

    /// Gets the [FirmwareVersion].
    pub fn firmware_version(&self) -> FirmwareVersion {
        self.buf[index::FIRMWARE_VERSION..index::FIRMWARE_VERSION_END]
            .as_ref()
            .into()
    }

    /// Gets the [CountryCode].
    pub fn country_code(&self) -> CountryCode {
        self.buf[index::COUNTRY_CODE..index::COUNTRY_CODE_END]
            .as_ref()
            .into()
    }

    /// Gets the [ValueMultiplier].
    pub fn value_multiplier(&self) -> ValueMultiplier {
        self.buf[index::VALUE_MULTIPLIER..index::VALUE_MULTIPLIER_END]
            .as_ref()
            .into()
    }

    /// Gets the [ProtocolVersion].
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.buf[index::PROTOCOL_VERSION].into()
    }
}

impl_default!(UnitDataResponse);
impl_message_from_buf!(UnitDataResponse);
impl_message_ops!(UnitDataResponse, MessageType::UnitData);
impl_response_ops!(UnitDataResponse);

impl fmt::Display for UnitDataResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let status = self.response_status();
        let unit_type = self.unit_type();
        let firmware_version = self.firmware_version();
        let country_code = self.country_code();
        let value_multiplier = self.value_multiplier();
        let protocol_version = self.protocol_version();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Unit type: {unit_type} | Firmware version: {firmware_version} | Country code: {country_code} | Value multiplier: {value_multiplier} | Protocol version: {protocol_version} | CRC-16: 0x{crc:04x}")
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_unit_data_response() -> Result<()> {
        use crate::{STX, ResponseStatus};

        let unit_data_response_bytes = [
            // STX(0x7f)
            STX,
            // SEQID
            0x80,
            // LEN
            0x0d, 
            // OK response
            0xf0,
            // Unit type
            0x00,
            // Firmware Version
            0x00, 0x33, 0x33, 0x33,
            // Country code (device)
            0x45, 0x55, 0x52,
            // Value multiplier
            0x00, 0x00, 0x01,
            // Protocol version
            0x07,
            // CRC-16
            0xa1, 0x51,
        ];

        let exp_response = ResponseStatus::Ok;
        let exp_unit_type = UnitType::from(0x00);
        let exp_firmware_version = FirmwareVersion::from(b"0333");
        let exp_country_code = CountryCode::from(b"EUR");
        let exp_value_multiplier = ValueMultiplier::from(1);
        let exp_protocol_version = ProtocolVersion::Seven;

        let response = UnitDataResponse::try_from(unit_data_response_bytes)?;

        assert_eq!(response.len(), unit_data_response_bytes.len());
        assert_eq!(response.data_len(), unit_data_response_bytes.len() - len::METADATA);

        assert_eq!(response.response_status(), exp_response);
        assert_eq!(response.unit_type(), exp_unit_type);
        assert_eq!(response.firmware_version(), exp_firmware_version);
        assert_eq!(response.country_code(), exp_country_code);
        assert_eq!(response.value_multiplier(), exp_value_multiplier);
        assert_eq!(response.protocol_version(), exp_protocol_version);

        Ok(())
    }
}
