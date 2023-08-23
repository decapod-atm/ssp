use crate::{
    std::fmt, ChannelValueDataResponse, ConfigureBezelResponse, DatasetVersionResponse,
    DisableResponse, DisplayOffResponse, DisplayOnResponse, EmptyResponse, EnablePayoutResponse,
    EnableResponse, EncryptionResetResponse, Error, EventAckResponse, GetBarcodeDataResponse,
    GetBarcodeInhibitResponse, GetBarcodeReaderConfigurationResponse, HoldResponse,
    HostProtocolVersionResponse, LastRejectCodeResponse, MessageType, PollResponse,
    PollWithAckResponse, RejectResponse, RequestKeyExchangeResponse, ResponseOps, Result,
    SerialNumberResponse, SetBarcodeInhibitResponse, SetBarcodeReaderConfigurationResponse,
    SetEncryptionKeyResponse, SetGeneratorResponse, SetInhibitsResponse, SetModulusResponse,
    SetupRequestResponse, SmartEmptyResponse, SyncResponse, UnitDataResponse,
    WrappedEncryptedMessage,
};

#[cfg(test)]
mod tests;

/// Message variant types
///
/// Used to construct response messages from byte buffers.
#[derive(Clone, Debug, PartialEq)]
pub enum MessageVariant {
    SetInhibitsResponse(SetInhibitsResponse),
    ChannelValueDataResponse(ChannelValueDataResponse),
    ConfigureBezelResponse(ConfigureBezelResponse),
    DatasetVersionResponse(DatasetVersionResponse),
    DisableResponse(DisableResponse),
    DisplayOffResponse(DisplayOffResponse),
    DisplayOnResponse(DisplayOnResponse),
    EmptyResponse(EmptyResponse),
    EncryptionResetResponse(EncryptionResetResponse),
    EnableResponse(EnableResponse),
    EnablePayoutResponse(EnablePayoutResponse),
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
    pub fn new(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::SetInhibits => Self::SetInhibitsResponse(SetInhibitsResponse::new()),
            MessageType::ChannelValueData => {
                Self::ChannelValueDataResponse(ChannelValueDataResponse::new())
            }
            MessageType::ConfigureBezel => {
                Self::ConfigureBezelResponse(ConfigureBezelResponse::new())
            }
            MessageType::DatasetVersion => {
                Self::DatasetVersionResponse(DatasetVersionResponse::new())
            }
            MessageType::Disable => Self::DisableResponse(DisableResponse::new()),
            MessageType::DisplayOff => Self::DisplayOffResponse(DisplayOffResponse::new()),
            MessageType::DisplayOn => Self::DisplayOnResponse(DisplayOnResponse::new()),
            MessageType::Empty => Self::EmptyResponse(EmptyResponse::new()),
            MessageType::EncryptionReset => {
                Self::EncryptionResetResponse(EncryptionResetResponse::new())
            }
            MessageType::Enable => Self::EnableResponse(EnableResponse::new()),
            MessageType::EnablePayout => Self::EnablePayoutResponse(EnablePayoutResponse::new()),
            MessageType::EventAck => Self::EventAckResponse(EventAckResponse::new()),
            MessageType::GetBarcodeData => {
                Self::GetBarcodeDataResponse(GetBarcodeDataResponse::new())
            }
            MessageType::GetBarcodeInhibit => {
                Self::GetBarcodeInhibitResponse(GetBarcodeInhibitResponse::new())
            }
            MessageType::GetBarcodeReaderConfiguration => {
                Self::GetBarcodeReaderConfigurationResponse(
                    GetBarcodeReaderConfigurationResponse::new(),
                )
            }
            MessageType::SetBarcodeInhibit => {
                Self::SetBarcodeInhibitResponse(SetBarcodeInhibitResponse::new())
            }
            MessageType::SetBarcodeReaderConfiguration => {
                Self::SetBarcodeReaderConfigurationResponse(
                    SetBarcodeReaderConfigurationResponse::new(),
                )
            }
            MessageType::Hold => Self::HoldResponse(HoldResponse::new()),
            MessageType::HostProtocolVersion => {
                Self::HostProtocolVersionResponse(HostProtocolVersionResponse::new())
            }
            MessageType::LastRejectCode => {
                Self::LastRejectCodeResponse(LastRejectCodeResponse::new())
            }
            MessageType::Poll => Self::PollResponse(PollResponse::new()),
            MessageType::PollWithAck => Self::PollWithAckResponse(PollWithAckResponse::new()),
            MessageType::Reject => Self::RejectResponse(RejectResponse::new()),
            MessageType::SerialNumber => Self::SerialNumberResponse(SerialNumberResponse::new()),
            MessageType::SetEncryptionKey => {
                Self::SetEncryptionKeyResponse(SetEncryptionKeyResponse::new())
            }
            MessageType::SetGenerator => Self::SetGeneratorResponse(SetGeneratorResponse::new()),
            MessageType::SetModulus => Self::SetModulusResponse(SetModulusResponse::new()),
            MessageType::RequestKeyExchange => {
                Self::RequestKeyExchangeResponse(RequestKeyExchangeResponse::new())
            }
            MessageType::SetupRequest => Self::SetupRequestResponse(SetupRequestResponse::new()),
            MessageType::SmartEmpty => Self::SmartEmptyResponse(SmartEmptyResponse::new()),
            MessageType::Synchronisation => Self::SyncResponse(SyncResponse::new()),
            MessageType::UnitData => Self::UnitDataResponse(UnitDataResponse::new()),
            MessageType::Encrypted => Self::WrappedEncryptedMessage(WrappedEncryptedMessage::new()),
            _ => Self::PollResponse(PollResponse::new()),
        }
    }

    /// Gets a reference to the [MessageVariant] as a generic response.
    pub fn as_response(&self) -> &dyn ResponseOps {
        match self {
            Self::SetInhibitsResponse(msg) => msg,
            Self::ChannelValueDataResponse(msg) => msg,
            Self::ConfigureBezelResponse(msg) => msg,
            Self::DatasetVersionResponse(msg) => msg,
            Self::DisableResponse(msg) => msg,
            Self::DisplayOffResponse(msg) => msg,
            Self::DisplayOnResponse(msg) => msg,
            Self::EmptyResponse(msg) => msg,
            Self::EnableResponse(msg) => msg,
            Self::EnablePayoutResponse(msg) => msg,
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

    /// Gets a mutable reference to the [MessageVariant] as a generic response.
    pub fn as_response_mut(&mut self) -> &mut dyn ResponseOps {
        match self {
            Self::SetInhibitsResponse(msg) => msg,
            Self::ChannelValueDataResponse(msg) => msg,
            Self::ConfigureBezelResponse(msg) => msg,
            Self::DatasetVersionResponse(msg) => msg,
            Self::DisableResponse(msg) => msg,
            Self::DisplayOffResponse(msg) => msg,
            Self::DisplayOnResponse(msg) => msg,
            Self::EmptyResponse(msg) => msg,
            Self::EnableResponse(msg) => msg,
            Self::EnablePayoutResponse(msg) => msg,
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
            MessageType::DatasetVersion => Ok(Self::DatasetVersionResponse(
                DatasetVersionResponse::try_from(buf)?,
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
            MessageType::EnablePayout => Ok(Self::EnablePayoutResponse(
                EnablePayoutResponse::try_from(buf)?,
            )),
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

inner_enum!(MessageVariant, SetInhibitsResponse);
inner_enum!(MessageVariant, ChannelValueDataResponse);
inner_enum!(MessageVariant, ConfigureBezelResponse);
inner_enum!(MessageVariant, DatasetVersionResponse);
inner_enum!(MessageVariant, DisableResponse);
inner_enum!(MessageVariant, DisplayOffResponse);
inner_enum!(MessageVariant, DisplayOnResponse);
inner_enum!(MessageVariant, EmptyResponse);
inner_enum!(MessageVariant, EncryptionResetResponse);
inner_enum!(MessageVariant, EnableResponse);
inner_enum!(MessageVariant, EnablePayoutResponse);
inner_enum!(MessageVariant, EventAckResponse);
inner_enum!(MessageVariant, GetBarcodeDataResponse);
inner_enum!(MessageVariant, GetBarcodeInhibitResponse);
inner_enum!(MessageVariant, GetBarcodeReaderConfigurationResponse);
inner_enum!(MessageVariant, SetBarcodeInhibitResponse);
inner_enum!(MessageVariant, SetBarcodeReaderConfigurationResponse);
inner_enum!(MessageVariant, HoldResponse);
inner_enum!(MessageVariant, HostProtocolVersionResponse);
inner_enum!(MessageVariant, LastRejectCodeResponse);
inner_enum!(MessageVariant, PollResponse);
inner_enum!(MessageVariant, PollWithAckResponse);
inner_enum!(MessageVariant, RejectResponse);
inner_enum!(MessageVariant, SerialNumberResponse);
inner_enum!(MessageVariant, SetEncryptionKeyResponse);
inner_enum!(MessageVariant, SetGeneratorResponse);
inner_enum!(MessageVariant, SetModulusResponse);
inner_enum!(MessageVariant, RequestKeyExchangeResponse);
inner_enum!(MessageVariant, SetupRequestResponse);
inner_enum!(MessageVariant, SmartEmptyResponse);
inner_enum!(MessageVariant, SyncResponse);
inner_enum!(MessageVariant, UnitDataResponse);
inner_enum!(MessageVariant, WrappedEncryptedMessage);

impl fmt::Display for MessageVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SetInhibitsResponse(msg) => write!(f, "SetInhibitsResponse({msg})"),
            Self::ChannelValueDataResponse(msg) => write!(f, "ChannelValueDataResponse({msg})"),
            Self::ConfigureBezelResponse(msg) => write!(f, "ConfigureBezelResponse({msg})"),
            Self::DatasetVersionResponse(msg) => write!(f, "DatasetVersionResponse({msg})"),
            Self::DisableResponse(msg) => write!(f, "DisableResponse({msg})"),
            Self::DisplayOffResponse(msg) => write!(f, "DisplayOffResponse({msg})"),
            Self::DisplayOnResponse(msg) => write!(f, "DisplayOnResponse({msg})"),
            Self::EmptyResponse(msg) => write!(f, "EmptyResponse({msg})"),
            Self::EnableResponse(msg) => write!(f, "EnableResponse({msg})"),
            Self::EnablePayoutResponse(msg) => write!(f, "EnablePayoutResponse({msg})"),
            Self::EncryptionResetResponse(msg) => write!(f, "EncryptionResetResponse({msg})"),
            Self::EventAckResponse(msg) => write!(f, "EventAckResponse({msg})"),
            Self::GetBarcodeDataResponse(msg) => write!(f, "GetBarcodeDataResponse({msg})"),
            Self::GetBarcodeInhibitResponse(msg) => write!(f, "GetBarcodeInhibitResponse({msg})"),
            Self::GetBarcodeReaderConfigurationResponse(msg) => {
                write!(f, "GetBarcodeReaderConfigurationResponse({msg})")
            }
            Self::SetBarcodeInhibitResponse(msg) => write!(f, "SetBarcodeInhibitResponse({msg})"),
            Self::SetBarcodeReaderConfigurationResponse(msg) => {
                write!(f, "SetBarcodeReaderConfigurationResponse({msg})")
            }
            Self::HoldResponse(msg) => write!(f, "HoldResponse({msg})"),
            Self::HostProtocolVersionResponse(msg) => {
                write!(f, "HostProtocolVersionResponse({msg})")
            }
            Self::LastRejectCodeResponse(msg) => write!(f, "LastRejectCodeResponse({msg})"),
            Self::PollResponse(msg) => write!(f, "PollResponse({msg})"),
            Self::PollWithAckResponse(msg) => write!(f, "PollWithAckResponse({msg})"),
            Self::RejectResponse(msg) => write!(f, "RejectResponse({msg})"),
            Self::SerialNumberResponse(msg) => write!(f, "SerialNumberResponse({msg})"),
            Self::SetEncryptionKeyResponse(msg) => write!(f, "SetEncryptionKeyResponse({msg})"),
            Self::SetGeneratorResponse(msg) => write!(f, "SetGeneratorResponse({msg})"),
            Self::SetModulusResponse(msg) => write!(f, "SetModulusResponse({msg})"),
            Self::RequestKeyExchangeResponse(msg) => write!(f, "RequestKeyExchangeResponse({msg})"),
            Self::SetupRequestResponse(msg) => write!(f, "SetupRequestResponse({msg})"),
            Self::SmartEmptyResponse(msg) => write!(f, "SmartEmptyResponse({msg})"),
            Self::SyncResponse(msg) => write!(f, "SyncResponse({msg})"),
            Self::UnitDataResponse(msg) => write!(f, "UnitDataResponse({msg})"),
            Self::WrappedEncryptedMessage(msg) => write!(f, "WrappedEncryptedMessage({msg})"),
        }
    }
}
