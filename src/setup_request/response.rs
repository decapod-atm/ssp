use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len, std::fmt,
    ChannelValue, ChannelValueList, CountryCode, CountryCodeList, Error, FirmwareVersion,
    MessageOps, MessageType, ProtocolVersion, ResponseOps, Result, UnitType, ValueMultiplier, Vec,
};

mod index {
    pub const UNIT_TYPE: usize = 4;
    pub const FIRMWARE_VERSION: usize = 5;
    pub const FIRMWARE_VERSION_END: usize = 9;
    pub const COUNTRY_CODE: usize = 9;
    pub const COUNTRY_CODE_END: usize = 12;
    pub const VALUE_MULTIPLIER: usize = 12;
    pub const VALUE_MULTIPLIER_END: usize = 15;
    pub const NUMBER_OF_CHANNELS: usize = 15;
    pub const CHANNEL_VALUES: usize = 16;
}

/// SetupRequest - Response (0x05)
///
/// Represents a response to an [SetupRequestCommand](crate::SetupRequestCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SetupRequestResponse {
    buf: [u8; len::SETUP_REQUEST_RESPONSE],
}

impl SetupRequestResponse {
    /// Creates a new [SetupRequestResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::SETUP_REQUEST_RESPONSE],
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

    /// Gets the [CountryCode] for the device.
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

    /// Gets the number of channels configured for the device.
    pub fn num_channels(&self) -> usize {
        self.buf[index::NUMBER_OF_CHANNELS] as usize
    }

    /// Gets the channel values (short, one-byte-per-channel).
    pub fn channel_values(&self) -> Result<ChannelValueList> {
        self.is_valid()?;

        let channel_values_end = self.channel_values_end();

        Ok(self.buf[index::CHANNEL_VALUES..channel_values_end]
            .iter()
            .map(|&v| ChannelValue::from([v]))
            .collect::<Vec<ChannelValue>>()
            .into())
    }

    fn channel_values_start(&self) -> usize {
        index::CHANNEL_VALUES
    }

    fn channel_values_end(&self) -> usize {
        self.channel_values_start() + self.num_channels()
    }

    /// Gets the channel security levels.
    pub fn channel_security_levels(&self) -> Result<&[u8]> {
        self.is_valid()?;

        let channel_sec_start = self.channel_security_levels_start();
        let channel_sec_end = self.channel_security_levels_end();

        Ok(self.buf[channel_sec_start..channel_sec_end].as_ref())
    }

    fn channel_security_levels_start(&self) -> usize {
        self.channel_values_end()
    }

    fn channel_security_levels_end(&self) -> usize {
        self.channel_security_levels_start() + self.num_channels()
    }

    /// Gets the real value multiplier.
    pub fn real_value_multiplier(&self) -> Result<ValueMultiplier> {
        self.is_valid()?;

        // real value multiplier starts after two fields with `num_channels` length.
        let real_value_start = self.real_value_multiplier_start();
        let real_value_end = self.real_value_multiplier_end();

        Ok(self.buf[real_value_start..real_value_end].as_ref().into())
    }

    fn real_value_multiplier_start(&self) -> usize {
        self.channel_security_levels_end()
    }

    fn real_value_multiplier_end(&self) -> usize {
        self.real_value_multiplier_start() + len::U24
    }

    /// Gets the currently configured [ProtocolVersion].
    pub fn protocol_version(&self) -> Result<ProtocolVersion> {
        self.is_valid()?;

        let protocol_version_idx = self.protocol_version_start();

        Ok(self.buf[protocol_version_idx].into())
    }

    fn protocol_version_start(&self) -> usize {
        self.real_value_multiplier_end()
    }

    fn protocol_version_end(&self) -> usize {
        self.protocol_version_start() + 1
    }

    /// Gets the [CountryCode] of each channel.
    pub fn channel_country_codes(&self) -> Result<CountryCodeList> {
        self.is_valid()?;

        let channel_country_code_start = self.channel_country_codes_start();
        // Each country code takes 3 bytes, one country code per channel.
        let channel_country_code_end = self.channel_country_codes_end();

        Ok(
            self.buf[channel_country_code_start..channel_country_code_end]
                .chunks_exact(len::U24)
                .map(CountryCode::from)
                .collect::<Vec<CountryCode>>()
                .into(),
        )
    }

    fn channel_country_codes_start(&self) -> usize {
        self.protocol_version_end()
    }

    fn channel_country_codes_end(&self) -> usize {
        self.channel_country_codes_start() + (len::U24 * self.num_channels())
    }

    /// Gets the [ChannelValue] of each channel (long, four-bytes-per-channel).
    pub fn channel_values_long(&self) -> Result<ChannelValueList> {
        self.is_valid()?;

        let channel_values_start = self.channel_values_long_start();
        let channel_values_end = self.channel_values_long_end();

        Ok(self.buf[channel_values_start..channel_values_end]
            .chunks_exact(4)
            .map(ChannelValue::from)
            .collect::<Vec<ChannelValue>>()
            .into())
    }

    fn channel_values_long_start(&self) -> usize {
        self.channel_country_codes_end()
    }

    fn channel_values_long_end(&self) -> usize {
        self.channel_values_long_start() + (4 * self.num_channels())
    }

    /// Gets whether the message contains a valid length and number of channels.
    ///
    /// Because the buffer has variable length, and fields that are multiples of the number of
    /// channels, there are combinations that result in overflowing the total buffer length.
    ///
    /// For example, LEN=255 & NUM_FIELDS=255 obviously overflows the buffer since the first
    /// CHANNEL_VALUES field is NUM_FIELDS*LEN_CHANNEL_VALUE(1) = 255.
    pub fn is_valid(&self) -> Result<()> {
        use crate::message::index as msg_index;

        let channel_values_start = self.channel_values_long_start();
        let channel_values_end = self.channel_values_long_end();
        let protocol_start = self.protocol_version_start();
        let protocol_end = self.protocol_version_end();

        let buf_end = len::HEADER + self.data_len();
        let buf_range = msg_index::DATA..=buf_end;

        let protocol = self
            .buf
            .get(protocol_start)
            .ok_or(Error::InvalidDataLength((protocol_start, buf_end)))?;

        let (start, end) = if *protocol < 6 {
            (protocol_start, protocol_end)
        } else {
            (channel_values_start, channel_values_end)
        };

        if buf_range.contains(&start) && buf_range.contains(&end) {
            Ok(())
        } else {
            Err(Error::InvalidDataLength((end, buf_end)))
        }
    }
}

impl_default!(SetupRequestResponse);
impl_message_from_buf!(SetupRequestResponse);
impl_var_message_ops!(SetupRequestResponse, MessageType::SetupRequest);
impl_response_ops!(SetupRequestResponse);

impl fmt::Display for SetupRequestResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_valid() {
            Ok(_) => {
                let stx = self.stx();
                let seqid = self.sequence_id();
                let data_len = self.data_len();
                let status = self.response_status();
                let unit = self.unit_type();
                let firmware_version = self.firmware_version();
                let country_code = self.country_code();
                let value_multi = self.value_multiplier();
                let num_channels = self.num_channels();
                let channel_values = self.channel_values().unwrap();
                let channel_sec = self.channel_security_levels().unwrap();
                let real_value_multi = self.real_value_multiplier().unwrap();
                let protocol_version = self.protocol_version().unwrap();
                let crc = self.checksum();

                write!(
                    f,
                    "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{data_len:02x} | Response status: {status} | Unit type: {unit} | Firmware version: {firmware_version} | Country code: {country_code} | Value multiplier: {value_multi} | Number of channels: {num_channels} | Channel values: {channel_values} | Channel security levels: {channel_sec:02x?} | Real value multiplier: {real_value_multi} | Protocol version: {protocol_version}")?;

                if (protocol_version as u8) < 6 {
                    write!(f, "| CRC-16: 0x{crc:04x}")
                } else {
                    let channel_country_codes = self.channel_country_codes().unwrap();
                    let channel_values_long = self.channel_values_long().unwrap();
                    write!(f, "| Channel country codes: {channel_country_codes} | Channel values (long): {channel_values_long} | CRC-16: 0x{crc:04x}")
                }
            }
            Err(err) => {
                write!(f, "Invalid SetupRequestResponse message: {err}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_setup_request_response() -> Result<()> {
        use crate::{STX, ResponseOps, ResponseStatus};

        let setup_request_response_bytes = [
            // STX(0x7f)
            STX,
            // SEQID
            0x80,
            // LEN
            0x35, 
            // OK response
            0xf0,
            // Validator type
            0x00,
            // Firmware Version
            0x00, 0x33, 0x33, 0x33,
            // Country code (device)
            0x45, 0x55, 0x52,
            // Value multiplier
            0x00, 0x00, 0x01,
            // Number of channels
            0x04,
            // Channel values (short)
            0x05, 0x0a, 0x14, 0x32,
            // Channel security levels
            0x02, 0x02, 0x02, 0x02,
            // Real value multiplier
            0x00, 0x00, 0x64,
            // Protocol version
            0x07,
            // Country codes (per-channel)
            0x45, 0x55, 0x52,
            0x45, 0x55, 0x52,
            0x45, 0x55, 0x52,
            0x45, 0x55, 0x52,
            // Channel values (long, 4 bytes-per-channel, little-endian)
            0x05, 0x00, 0x00, 0x00,
            0x0a, 0x00, 0x00, 0x00,
            0x14, 0x00, 0x00, 0x00,
            0x32, 0x00, 0x00, 0x00,
            // CRC-16
            0x8b, 0x9b,
        ];

        let exp_response = ResponseStatus::Ok;
        let exp_unit_type = UnitType::from(0x00);
        let exp_firmware_version = FirmwareVersion::from(b"0333");
        let exp_country_code = CountryCode::from(b"EUR");
        let exp_value_multiplier = ValueMultiplier::from(1);
        let exp_num_channels = 4;
        let exp_channel_values = [
            ChannelValue::from(5),
            ChannelValue::from(10),
            ChannelValue::from(20),
            ChannelValue::from(50),
        ];
        let exp_channel_security = [2, 2, 2, 2];
        let exp_real_value_multiplier = ValueMultiplier::from(100);
        let exp_protocol_version = ProtocolVersion::Seven;
        let exp_country_codes = [
            CountryCode::from(b"EUR"),
            CountryCode::from(b"EUR"),
            CountryCode::from(b"EUR"),
            CountryCode::from(b"EUR"),
        ];
        let exp_channel_values_long = [
            ChannelValue::from(5),
            ChannelValue::from(10),
            ChannelValue::from(20),
            ChannelValue::from(50),
        ];

        let response = SetupRequestResponse::try_from(setup_request_response_bytes)?;

        assert_eq!(response.len(), setup_request_response_bytes.len());
        assert_eq!(response.data_len(), setup_request_response_bytes.len() - len::METADATA);

        assert_eq!(response.response_status(), exp_response);
        assert_eq!(response.unit_type(), exp_unit_type);
        assert_eq!(response.firmware_version(), exp_firmware_version);
        assert_eq!(response.country_code(), exp_country_code);
        assert_eq!(response.value_multiplier(), exp_value_multiplier);
        assert_eq!(response.num_channels(), exp_num_channels);
        assert_eq!(response.channel_values()?.as_ref(), exp_channel_values.as_ref());
        assert_eq!(response.channel_security_levels()?, exp_channel_security.as_ref());
        assert_eq!(response.real_value_multiplier()?, exp_real_value_multiplier);
        assert_eq!(response.protocol_version()?, exp_protocol_version);
        assert_eq!(response.channel_country_codes()?.as_ref(), exp_country_codes.as_ref());
        assert_eq!(response.channel_values_long()?.as_ref(), exp_channel_values_long.as_ref());

        Ok(())
    }
}
