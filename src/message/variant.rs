use crate::{
    ChannelValueDataResponse, ConfigureBezelResponse, DisableResponse, DisplayOffResponse,
    DisplayOnResponse, EmptyResponse, EnableResponse, EncryptionResetResponse, Error,
    EventAckResponse, GetBarcodeDataResponse, GetBarcodeInhibitResponse,
    GetBarcodeReaderConfigurationResponse, HoldResponse, HostProtocolVersionResponse,
    LastRejectCodeResponse, MessageType, PollResponse, PollWithAckResponse, RejectResponse,
    RequestKeyExchangeResponse, ResponseOps, Result, SerialNumberResponse,
    SetBarcodeInhibitResponse, SetBarcodeReaderConfigurationResponse, SetEncryptionKeyResponse,
    SetGeneratorResponse, SetInhibitsResponse, SetModulusResponse, SetupRequestResponse,
    SmartEmptyResponse, SyncResponse, UnitDataResponse, WrappedEncryptedMessage,
};

/// Message variant types
///
/// Used to construct response messages from byte buffers.
#[derive(Clone, Debug, PartialEq)]
pub enum MessageVariant {
    SetInhibitsResponse(SetInhibitsResponse),
    ChannelValueDataResponse(ChannelValueDataResponse),
    ConfigureBezelResponse(ConfigureBezelResponse),
    DisableResponse(DisableResponse),
    DisplayOffResponse(DisplayOffResponse),
    DisplayOnResponse(DisplayOnResponse),
    EmptyResponse(EmptyResponse),
    EncryptionResetResponse(EncryptionResetResponse),
    EnableResponse(EnableResponse),
    EventAckResponse(EventAckResponse),
    GetBarcodeDataResponse(GetBarcodeDataResponse),
    GetBarcodeInhibitResponse(GetBarcodeInhibitResponse),
    GetBarcodeReaderConfigurationResponse(GetBarcodeReaderConfigurationResponse),
    SetBarcodeInhibitResponse(SetBarcodeInhibitResponse),
    SetBarcodeReaderConfigurationResponse(SetBarcodeReaderConfigurationResponse),
    HoldResponse(HoldResponse),
    HostProtocolVersionResponse(HostProtocolVersionResponse),
    LastRejectCodeResponse(LastRejectCodeResponse),
    PollResponse(PollResponse),
    PollWithAckResponse(PollWithAckResponse),
    RejectResponse(RejectResponse),
    SerialNumberResponse(SerialNumberResponse),
    SetEncryptionKeyResponse(SetEncryptionKeyResponse),
    SetGeneratorResponse(SetGeneratorResponse),
    SetModulusResponse(SetModulusResponse),
    RequestKeyExchangeResponse(RequestKeyExchangeResponse),
    SetupRequestResponse(SetupRequestResponse),
    SmartEmptyResponse(SmartEmptyResponse),
    SyncResponse(SyncResponse),
    UnitDataResponse(UnitDataResponse),
    WrappedEncryptedMessage(WrappedEncryptedMessage),
}

impl MessageVariant {
    /// Gets a reference to the [MessageVariant] as a generic response.
    pub fn as_response(&self) -> &dyn ResponseOps {
        match self {
            Self::SetInhibitsResponse(msg) => msg,
            Self::ChannelValueDataResponse(msg) => msg,
            Self::ConfigureBezelResponse(msg) => msg,
            Self::DisableResponse(msg) => msg,
            Self::DisplayOffResponse(msg) => msg,
            Self::DisplayOnResponse(msg) => msg,
            Self::EmptyResponse(msg) => msg,
            Self::EnableResponse(msg) => msg,
            Self::EncryptionResetResponse(msg) => msg,
            Self::EventAckResponse(msg) => msg,
            Self::GetBarcodeDataResponse(msg) => msg,
            Self::GetBarcodeInhibitResponse(msg) => msg,
            Self::GetBarcodeReaderConfigurationResponse(msg) => msg,
            Self::SetBarcodeInhibitResponse(msg) => msg,
            Self::SetBarcodeReaderConfigurationResponse(msg) => msg,
            Self::HoldResponse(msg) => msg,
            Self::HostProtocolVersionResponse(msg) => msg,
            Self::LastRejectCodeResponse(msg) => msg,
            Self::PollResponse(msg) => msg,
            Self::PollWithAckResponse(msg) => msg,
            Self::RejectResponse(msg) => msg,
            Self::SerialNumberResponse(msg) => msg,
            Self::SetEncryptionKeyResponse(msg) => msg,
            Self::SetGeneratorResponse(msg) => msg,
            Self::SetModulusResponse(msg) => msg,
            Self::RequestKeyExchangeResponse(msg) => msg,
            Self::SetupRequestResponse(msg) => msg,
            Self::SmartEmptyResponse(msg) => msg,
            Self::SyncResponse(msg) => msg,
            Self::UnitDataResponse(msg) => msg,
            Self::WrappedEncryptedMessage(msg) => msg,
        }
    }

    /// Gets whether the [MessageVariant] contains a [SetInhibitsResponse] message.
    pub fn is_set_inhibits_response(&self) -> bool {
        matches!(self, Self::SetInhibitsResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [SetInhibitsResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetInhibitsResponse].
    pub fn as_set_inhibits_response(&self) -> Result<&SetInhibitsResponse> {
        match self {
            Self::SetInhibitsResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetInhibitsResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetInhibitsResponse].
    pub fn into_set_inhibits_response(self) -> Result<SetInhibitsResponse> {
        match self {
            Self::SetInhibitsResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [ChannelValueDataResponse] message.
    pub fn is_channel_value_data_response(&self) -> bool {
        matches!(self, Self::ChannelValueDataResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [ChannelValueDataResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [ChannelValueDataResponse].
    pub fn as_channel_value_data_response(&self) -> Result<&ChannelValueDataResponse> {
        match self {
            Self::ChannelValueDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [ChannelValueDataResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [ChannelValueDataResponse].
    pub fn into_channel_value_data_response(self) -> Result<ChannelValueDataResponse> {
        match self {
            Self::ChannelValueDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [ConfigureBezelResponse] message.
    pub fn is_configure_bezel_response(&self) -> bool {
        matches!(self, Self::ConfigureBezelResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [ConfigureBezelResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [ConfigureBezelResponse].
    pub fn as_configure_bezel_response(&self) -> Result<&ConfigureBezelResponse> {
        match self {
            Self::ConfigureBezelResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [ConfigureBezelResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [ConfigureBezelResponse].
    pub fn into_configure_bezel_response(self) -> Result<ConfigureBezelResponse> {
        match self {
            Self::ConfigureBezelResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [DisableResponse] message.
    pub fn is_disable_response(&self) -> bool {
        matches!(self, Self::DisableResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [DisableResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisableResponse].
    pub fn as_disable_response(&self) -> Result<&DisableResponse> {
        match self {
            Self::DisableResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [DisableResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisableResponse].
    pub fn into_disable_response(self) -> Result<DisableResponse> {
        match self {
            Self::DisableResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [DisplayOffResponse] message.
    pub fn is_display_off_response(&self) -> bool {
        matches!(self, Self::DisplayOffResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [DisplayOffResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisplayOffResponse].
    pub fn as_display_off_response(&self) -> Result<&DisplayOffResponse> {
        match self {
            Self::DisplayOffResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [DisplayOffResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisplayOffResponse].
    pub fn into_display_off_response(self) -> Result<DisplayOffResponse> {
        match self {
            Self::DisplayOffResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [DisplayOnResponse] message.
    pub fn is_display_on_response(&self) -> bool {
        matches!(self, Self::DisplayOnResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [DisplayOnResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisplayOnResponse].
    pub fn as_display_on_response(&self) -> Result<&DisplayOnResponse> {
        match self {
            Self::DisplayOnResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [DisplayOnResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [DisplayOnResponse].
    pub fn into_display_on_response(self) -> Result<DisplayOnResponse> {
        match self {
            Self::DisplayOnResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [EmptyResponse] message.
    pub fn is_empty_response(&self) -> bool {
        matches!(self, Self::EmptyResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [EmptyResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EmptyResponse].
    pub fn as_empty_response(&self) -> Result<&EmptyResponse> {
        match self {
            Self::EmptyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [EmptyResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EmptyResponse].
    pub fn into_empty_response(self) -> Result<EmptyResponse> {
        match self {
            Self::EmptyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SmartEmptyResponse] message.
    pub fn is_smart_empty_response(&self) -> bool {
        matches!(self, Self::SmartEmptyResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SmartEmptyResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SmartEmptyResponse].
    pub fn as_smart_empty_response(&self) -> Result<&SmartEmptyResponse> {
        match self {
            Self::SmartEmptyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SmartEmptyResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SmartEmptyResponse].
    pub fn into_smart_empty_response(self) -> Result<SmartEmptyResponse> {
        match self {
            Self::SmartEmptyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [EnableResponse] message.
    pub fn is_enable_response(&self) -> bool {
        matches!(self, Self::EnableResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [EnableResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EnableResponse].
    pub fn as_enable_response(&self) -> Result<&EnableResponse> {
        match self {
            Self::EnableResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [EnableResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EnableResponse].
    pub fn into_enable_response(self) -> Result<EnableResponse> {
        match self {
            Self::EnableResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [EncryptionResetResponse] message.
    pub fn is_encryption_reset_response(&self) -> bool {
        matches!(self, Self::EncryptionResetResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [EncryptionResetResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EncryptionResetResponse].
    pub fn as_encryption_reset_response(&self) -> Result<&EncryptionResetResponse> {
        match self {
            Self::EncryptionResetResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [EncryptionResetResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EncryptionResetResponse].
    pub fn into_encryption_reset_response(self) -> Result<EncryptionResetResponse> {
        match self {
            Self::EncryptionResetResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [EventAckResponse] message.
    pub fn is_event_ack_response(&self) -> bool {
        matches!(self, Self::EventAckResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [EventAckResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EventAckResponse].
    pub fn as_event_ack_response(&self) -> Result<&EventAckResponse> {
        match self {
            Self::EventAckResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [EventAckResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [EventAckResponse].
    pub fn into_event_ack_response(self) -> Result<EventAckResponse> {
        match self {
            Self::EventAckResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [GetBarcodeDataResponse] message.
    pub fn is_get_barcode_data_response(&self) -> bool {
        matches!(self, Self::GetBarcodeDataResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [GetBarcodeDataResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeDataResponse].
    pub fn as_get_barcode_data_response(&self) -> Result<&GetBarcodeDataResponse> {
        match self {
            Self::GetBarcodeDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [GetBarcodeDataResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeDataResponse].
    pub fn into_get_barcode_data_response(self) -> Result<GetBarcodeDataResponse> {
        match self {
            Self::GetBarcodeDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [GetBarcodeInhibitResponse] message.
    pub fn is_get_barcode_inhibit_response(&self) -> bool {
        matches!(self, Self::GetBarcodeInhibitResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [GetBarcodeInhibitResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeInhibitResponse].
    pub fn as_get_barcode_inhibit_response(&self) -> Result<&GetBarcodeInhibitResponse> {
        match self {
            Self::GetBarcodeInhibitResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [GetBarcodeInhibitResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeInhibitResponse].
    pub fn into_get_barcode_inhibit_response(self) -> Result<GetBarcodeInhibitResponse> {
        match self {
            Self::GetBarcodeInhibitResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [GetBarcodeReaderConfigurationResponse] message.
    pub fn is_get_barcode_reader_configuration_response(&self) -> bool {
        matches!(self, Self::GetBarcodeReaderConfigurationResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [GetBarcodeReaderConfigurationResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeReaderConfigurationResponse].
    pub fn as_get_barcode_reader_configuration_response(
        &self,
    ) -> Result<&GetBarcodeReaderConfigurationResponse> {
        match self {
            Self::GetBarcodeReaderConfigurationResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [GetBarcodeReaderConfigurationResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [GetBarcodeReaderConfigurationResponse].
    pub fn into_get_barcode_reader_configuration_response(
        self,
    ) -> Result<GetBarcodeReaderConfigurationResponse> {
        match self {
            Self::GetBarcodeReaderConfigurationResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Sets whether the [MessageVariant] contains an [GetBarcodeInhibitResponse] message.
    pub fn is_set_barcode_inhibit_response(&self) -> bool {
        matches!(self, Self::SetBarcodeInhibitResponse(_))
    }

    /// Sets a reference to the [MessageVariant] as an [GetBarcodeInhibitResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetBarcodeInhibitResponse].
    pub fn as_set_barcode_inhibit_response(&self) -> Result<&SetBarcodeInhibitResponse> {
        match self {
            Self::SetBarcodeInhibitResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetBarcodeInhibitResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetBarcodeInhibitResponse].
    pub fn into_set_barcode_inhibit_response(self) -> Result<SetBarcodeInhibitResponse> {
        match self {
            Self::SetBarcodeInhibitResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SetBarcodeReaderConfigurationResponse] message.
    pub fn is_set_barcode_reader_configuration_response(&self) -> bool {
        matches!(self, Self::SetBarcodeReaderConfigurationResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SetBarcodeReaderConfigurationResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetBarcodeReaderConfigurationResponse].
    pub fn as_set_barcode_reader_configuration_response(
        &self,
    ) -> Result<&SetBarcodeReaderConfigurationResponse> {
        match self {
            Self::SetBarcodeReaderConfigurationResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetBarcodeReaderConfigurationResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetBarcodeReaderConfigurationResponse].
    pub fn into_set_barcode_reader_configuration_response(
        self,
    ) -> Result<SetBarcodeReaderConfigurationResponse> {
        match self {
            Self::SetBarcodeReaderConfigurationResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [HoldResponse] message.
    pub fn is_hold_response(&self) -> bool {
        matches!(self, Self::HoldResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [HoldResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [HoldResponse].
    pub fn as_hold_response(&self) -> Result<&HoldResponse> {
        match self {
            Self::HoldResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [HoldResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [HoldResponse].
    pub fn into_hold_response(self) -> Result<HoldResponse> {
        match self {
            Self::HoldResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [HostProtocolVersionResponse] message.
    pub fn is_host_protocol_version_response(&self) -> bool {
        matches!(self, Self::HostProtocolVersionResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [HostProtocolVersionResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [HostProtocolVersionResponse].
    pub fn as_host_protocol_version_response(&self) -> Result<&HostProtocolVersionResponse> {
        match self {
            Self::HostProtocolVersionResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [HostProtocolVersionResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [HostProtocolVersionResponse].
    pub fn into_host_protocol_version_response(self) -> Result<HostProtocolVersionResponse> {
        match self {
            Self::HostProtocolVersionResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [LastRejectCodeResponse] message.
    pub fn is_last_reject_code_response(&self) -> bool {
        matches!(self, Self::LastRejectCodeResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [LastRejectCodeResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [LastRejectCodeResponse].
    pub fn as_last_reject_code_response(&self) -> Result<&LastRejectCodeResponse> {
        match self {
            Self::LastRejectCodeResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [LastRejectCodeResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [LastRejectCodeResponse].
    pub fn into_last_reject_code_response(self) -> Result<LastRejectCodeResponse> {
        match self {
            Self::LastRejectCodeResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [PollResponse] message.
    pub fn is_poll_response(&self) -> bool {
        matches!(self, Self::PollResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [PollResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [PollResponse].
    pub fn as_poll_response(&self) -> Result<&PollResponse> {
        match self {
            Self::PollResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [PollResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [PollResponse].
    pub fn into_poll_response(self) -> Result<PollResponse> {
        match self {
            Self::PollResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains a [PollWithAckResponse] message.
    pub fn is_poll_with_ack_response(&self) -> bool {
        matches!(self, Self::PollWithAckResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as a [PollWithAckResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [PollWithAckResponse].
    pub fn as_poll_with_ack_response(&self) -> Result<&PollWithAckResponse> {
        match self {
            Self::PollWithAckResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [PollWithAckResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [PollWithAckResponse].
    pub fn into_poll_with_ack_response(self) -> Result<PollWithAckResponse> {
        match self {
            Self::PollWithAckResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [RejectResponse] message.
    pub fn is_reject_response(&self) -> bool {
        matches!(self, Self::RejectResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [RejectResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [RejectResponse].
    pub fn as_reject_response(&self) -> Result<&RejectResponse> {
        match self {
            Self::RejectResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [RejectResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [RejectResponse].
    pub fn into_reject_response(self) -> Result<RejectResponse> {
        match self {
            Self::RejectResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SerialNumberResponse] message.
    pub fn is_serial_number_response(&self) -> bool {
        matches!(self, Self::SerialNumberResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SerialNumberResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SerialNumberResponse].
    pub fn as_serial_number_response(&self) -> Result<&SerialNumberResponse> {
        match self {
            Self::SerialNumberResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SerialNumberResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SerialNumberResponse].
    pub fn into_serial_number_response(self) -> Result<SerialNumberResponse> {
        match self {
            Self::SerialNumberResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SetEncryptionKeyResponse] message.
    pub fn is_set_encryption_key_response(&self) -> bool {
        matches!(self, Self::SetEncryptionKeyResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SetEncryptionKeyResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetEncryptionKeyResponse].
    pub fn as_set_encryption_key_response(&self) -> Result<&SetEncryptionKeyResponse> {
        match self {
            Self::SetEncryptionKeyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetEncryptionKeyResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetEncryptionKeyResponse].
    pub fn into_set_encryption_key_response(self) -> Result<SetEncryptionKeyResponse> {
        match self {
            Self::SetEncryptionKeyResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SetGeneratorResponse] message.
    pub fn is_set_generator_response(&self) -> bool {
        matches!(self, Self::SetGeneratorResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SetGeneratorResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetGeneratorResponse].
    pub fn as_set_generator_response(&self) -> Result<&SetGeneratorResponse> {
        match self {
            Self::SetGeneratorResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetGeneratorResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetGeneratorResponse].
    pub fn into_set_generator_response(self) -> Result<SetGeneratorResponse> {
        match self {
            Self::SetGeneratorResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SetModulusResponse] message.
    pub fn is_set_modulus_response(&self) -> bool {
        matches!(self, Self::SetModulusResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SetModulusResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetModulusResponse].
    pub fn as_set_modulus_response(&self) -> Result<&SetModulusResponse> {
        match self {
            Self::SetModulusResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetModulusResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetModulusResponse].
    pub fn into_set_modulus_response(self) -> Result<SetModulusResponse> {
        match self {
            Self::SetModulusResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [RequestKeyExchangeResponse] message.
    pub fn is_request_key_exchange_response(&self) -> bool {
        matches!(self, Self::RequestKeyExchangeResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [RequestKeyExchangeResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [RequestKeyExchangeResponse].
    pub fn as_request_key_exchange_response(&self) -> Result<&RequestKeyExchangeResponse> {
        match self {
            Self::RequestKeyExchangeResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [RequestKeyExchangeResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [RequestKeyExchangeResponse].
    pub fn into_request_key_exchange_response(self) -> Result<RequestKeyExchangeResponse> {
        match self {
            Self::RequestKeyExchangeResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SetupRequestResponse] message.
    pub fn is_setup_request_response(&self) -> bool {
        matches!(self, Self::SetupRequestResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SetupRequestResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetupRequestResponse].
    pub fn as_setup_request_response(&self) -> Result<&SetupRequestResponse> {
        match self {
            Self::SetupRequestResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SetupRequestResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SetupRequestResponse].
    pub fn into_setup_request_response(self) -> Result<SetupRequestResponse> {
        match self {
            Self::SetupRequestResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [SyncResponse] message.
    pub fn is_sync_response(&self) -> bool {
        matches!(self, Self::SyncResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [SyncResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SyncResponse].
    pub fn as_sync_response(&self) -> Result<&SyncResponse> {
        match self {
            Self::SyncResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [SyncResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [SyncResponse].
    pub fn into_sync_response(self) -> Result<SyncResponse> {
        match self {
            Self::SyncResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [UnitDataResponse] message.
    pub fn is_unit_data_response(&self) -> bool {
        matches!(self, Self::UnitDataResponse(_))
    }

    /// Gets a reference to the [MessageVariant] as an [UnitDataResponse].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [UnitDataResponse].
    pub fn as_unit_data_response(&self) -> Result<&UnitDataResponse> {
        match self {
            Self::UnitDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [UnitDataResponse].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [UnitDataResponse].
    pub fn into_unit_data_response(self) -> Result<UnitDataResponse> {
        match self {
            Self::UnitDataResponse(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets whether the [MessageVariant] contains an [WrappedEncryptedMessage] message.
    pub fn is_wrapped_encrypted_message(&self) -> bool {
        matches!(self, Self::WrappedEncryptedMessage(_))
    }

    /// Gets a reference to the [MessageVariant] as an [WrappedEncryptedMessage].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [WrappedEncryptedMessage].
    pub fn as_wrapped_encrypted_message(&self) -> Result<&WrappedEncryptedMessage> {
        match self {
            Self::WrappedEncryptedMessage(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Converts the [MessageVariant] into an [WrappedEncryptedMessage].
    ///
    /// Consumes the [MessageVariant].
    ///
    /// Returns an [Error] if the [MessageVariant] does not contain a [WrappedEncryptedMessage].
    pub fn into_wrapped_encrypted_message(self) -> Result<WrappedEncryptedMessage> {
        match self {
            Self::WrappedEncryptedMessage(msg) => Ok(msg),
            _ => Err(Error::InvalidMessage(self.message_type())),
        }
    }

    /// Gets the current message type of the [MessageVariant].
    pub fn message_type(&self) -> MessageType {
        self.as_response().message_type()
    }

    /// Converts a byte buffer and [MessageType] into a [MessageVariant].
    pub fn from_buf(buf: &[u8], command_type: MessageType) -> Result<Self> {
        match command_type {
            MessageType::SetInhibits => Ok(Self::SetInhibitsResponse(
                SetInhibitsResponse::try_from(buf)?,
            )),
            MessageType::ChannelValueData => Ok(Self::ChannelValueDataResponse(
                ChannelValueDataResponse::try_from(buf)?,
            )),
            MessageType::ConfigureBezel => Ok(Self::ConfigureBezelResponse(
                ConfigureBezelResponse::try_from(buf)?,
            )),
            MessageType::Disable => Ok(Self::DisableResponse(DisableResponse::try_from(buf)?)),
            MessageType::DisplayOff => {
                Ok(Self::DisplayOffResponse(DisplayOffResponse::try_from(buf)?))
            }
            MessageType::DisplayOn => {
                Ok(Self::DisplayOnResponse(DisplayOnResponse::try_from(buf)?))
            }
            MessageType::Empty => Ok(Self::EmptyResponse(EmptyResponse::try_from(buf)?)),
            MessageType::Enable => Ok(Self::EnableResponse(EnableResponse::try_from(buf)?)),
            MessageType::EncryptionReset => Ok(Self::EncryptionResetResponse(
                EncryptionResetResponse::try_from(buf)?,
            )),
            MessageType::EventAck => Ok(Self::EventAckResponse(EventAckResponse::try_from(buf)?)),
            MessageType::GetBarcodeData => Ok(Self::GetBarcodeDataResponse(
                GetBarcodeDataResponse::try_from(buf)?,
            )),
            MessageType::GetBarcodeInhibit => Ok(Self::GetBarcodeInhibitResponse(
                GetBarcodeInhibitResponse::try_from(buf)?,
            )),
            MessageType::GetBarcodeReaderConfiguration => {
                Ok(Self::GetBarcodeReaderConfigurationResponse(
                    GetBarcodeReaderConfigurationResponse::try_from(buf)?,
                ))
            }
            MessageType::SetBarcodeInhibit => Ok(Self::SetBarcodeInhibitResponse(
                SetBarcodeInhibitResponse::try_from(buf)?,
            )),
            MessageType::SetBarcodeReaderConfiguration => {
                Ok(Self::SetBarcodeReaderConfigurationResponse(
                    SetBarcodeReaderConfigurationResponse::try_from(buf)?,
                ))
            }
            MessageType::Hold => Ok(Self::HoldResponse(HoldResponse::try_from(buf)?)),
            MessageType::HostProtocolVersion => Ok(Self::HostProtocolVersionResponse(
                HostProtocolVersionResponse::try_from(buf)?,
            )),
            MessageType::LastRejectCode => Ok(Self::LastRejectCodeResponse(
                LastRejectCodeResponse::try_from(buf)?,
            )),
            MessageType::Poll => Ok(Self::PollResponse(PollResponse::try_from(buf)?)),
            MessageType::PollWithAck => Ok(Self::PollWithAckResponse(
                PollWithAckResponse::try_from(buf)?,
            )),
            MessageType::Reject => Ok(Self::RejectResponse(RejectResponse::try_from(buf)?)),
            MessageType::SerialNumber => Ok(Self::SerialNumberResponse(
                SerialNumberResponse::try_from(buf)?,
            )),
            MessageType::SetEncryptionKey => Ok(Self::SetEncryptionKeyResponse(
                SetEncryptionKeyResponse::try_from(buf)?,
            )),
            MessageType::SetGenerator => Ok(Self::SetGeneratorResponse(
                SetGeneratorResponse::try_from(buf)?,
            )),
            MessageType::SetModulus => {
                Ok(Self::SetModulusResponse(SetModulusResponse::try_from(buf)?))
            }
            MessageType::RequestKeyExchange => Ok(Self::RequestKeyExchangeResponse(
                RequestKeyExchangeResponse::try_from(buf)?,
            )),
            MessageType::SetupRequest => Ok(Self::SetupRequestResponse(
                SetupRequestResponse::try_from(buf)?,
            )),
            MessageType::SmartEmpty => {
                Ok(Self::SmartEmptyResponse(SmartEmptyResponse::try_from(buf)?))
            }
            MessageType::Synchronisation => Ok(Self::SyncResponse(SyncResponse::try_from(buf)?)),
            MessageType::UnitData => Ok(Self::UnitDataResponse(UnitDataResponse::try_from(buf)?)),
            MessageType::Encrypted => Ok(Self::WrappedEncryptedMessage(
                WrappedEncryptedMessage::try_from(buf)?,
            )),
            _ => Err(Error::InvalidMessage(command_type)),
        }
    }
}
