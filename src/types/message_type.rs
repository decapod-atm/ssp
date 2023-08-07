use crate::{impl_default, std::fmt};

/// Type of command and response message.
///
/// Only command messages include the type of message in the message buffer itself.
///
/// Response types are inferred based on the type of command sent to the device.
///
/// The inference can fail. For example, if sending a command with the same
/// [SequenceFlag](crate::SequenceFlag) value as the previous message, the device will return the
/// response to the previous message. This happens regardless of the combination of message types.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageType {
    /// Resets the device.
    Reset = 0x01,
    /// Sets the note inhibits for the device. Establishes which notes are accepted.
    SetInhibits = 0x02,
    /// Single byte command turns on the bezel light when the unit is enabled.
    DisplayOn = 0x03,
    /// Single byte command turns off the bezel light when the unit is enabled.
    DisplayOff = 0x04,
    /// Single byte command to request the current setup configuration for the device.
    SetupRequest = 0x05,
    /// Host protocol version.
    ///
    /// Two byte command sets the unit to report events up to and including those found in the
    /// specified protocol version. Please note that the highest protocol version that a unit will
    /// support is determined by its firmware.
    HostProtocolVersion = 0x06,
    /// Standard polling command.
    ///
    /// Single byte command instructs the unit to report all the events that have occurred since the
    /// last time a poll was sent to the unit.
    ///
    /// Also, represents a `Stack Note` command when a note is reported in escrow.
    Poll = 0x07,
    /// Returns the note to the bezel.
    ///
    /// Single byte command causes the validator to reject the current note
    Reject = 0x08,
    /// This single byte command disables the unit. This means the unit will enter its disabled state
    /// and not execute any further commands or perform any other actions. A poll to the unit
    /// while in this state will report disabled (0xE8).
    Disable = 0x09,
    /// Single byte command enables the unit. It will now respond to and execute commands.
    Enable = 0x0a,
    /// Starts the program firmware mode to install new firmware on the device.
    ProgramFirmware = 0x0b,
    /// Gets the device's serial number.
    SerialNumber = 0x0c,
    /// A more concise version of the [SetupRequest](Self::SetupRequest) command. Returns unit
    /// information about the device.
    UnitData = 0x0d,
    /// Causes the validator to return the number of channels it is using, and the value for each
    /// channel.
    ChannelValueData = 0x0e,
    /// This single byte command tells the unit that the next sequence ID will be 1. This is always
    /// the first command sent to a unit, to prepare it to receive any further commands.
    Synchronisation = 0x11,
    /// Gets the last Reject Code issued by the device.
    LastRejectCode = 0x17,
    /// Keep the note in escrow position (for 5 seconds longer).
    Hold = 0x18,
    /// Gets the firmware version of the device.
    FirmwareVersion = 0x20,
    /// Gets the dataset version of the device.
    DatasetVersion = 0x21,
    /// Set the configuration status of any barcode readers present on the device.
    SetBarcodeReaderConfiguration = 0x23,
    /// Get the configuration status of any barcode readers present on the device.
    GetBarcodeReaderConfiguration = 0x24,
    /// Get the current barcode/currency mode settings.
    GetBarcodeInhibit = 0x25,
    /// Set the current barcode/currency mode settings.
    SetBarcodeInhibit = 0x26,
    /// Get the last valid barcode ticket data.
    GetBarcodeData = 0x27,
    /// Gets the manufacturer's extension of the device.
    ManufacturerExtension = 0x30,
    /// Causes the SMART Payout to empty all its stored notes to the cashbox.
    Empty = 0x3f,
    /// Payout notes to the customer by denomination.
    PayoutByDenomination = 0x46,
    /// Sets the eSSP generator prime (64-bits).
    SetGenerator = 0x4a,
    /// Sets the eSSP modulus (64-bits).
    SetModulus = 0x4b,
    /// Requests a Difie-Hellman key exchange to establish a shared secret for eSSP, i.e. the AES
    /// key.
    RequestKeyExchange = 0x4c,
    /// Empty the stored notes from the device into the cash box, keeping track of the values.
    SmartEmpty = 0x52,
    /// Configure the bezel color for the device.
    ConfigureBezel = 0x54,
    /// Poll the device, requires host to ACK returned events.
    PollWithAck = 0x56,
    /// Causes the validator to continue processing after returning repeating
    /// [PollWithAckResponse](crate::PollWithAckResponse) messages.
    EventAck = 0x57,
    /// SSP Set Encryption Key - sets the fixed encryption key to the user-supplied value.
    SetEncryptionKey = 0x60,
    /// SSP Encryption Reset to Default - resets the fixed encryption key to the default value.
    EncryptionReset = 0x61,
    /// Encrypted (eSSP) message - an encrypted version of another message.
    Encrypted = 0x7e,
    /// Reserved for future use.
    Reserved = 0xff,
}

impl MessageType {
    /// Creates a new [MessageType].
    pub const fn new() -> Self {
        Self::Reserved
    }
}

impl From<u8> for MessageType {
    fn from(b: u8) -> Self {
        match b {
            0x01 => Self::Reset,
            0x02 => Self::SetInhibits,
            0x03 => Self::DisplayOn,
            0x04 => Self::DisplayOff,
            0x05 => Self::SetupRequest,
            0x06 => Self::HostProtocolVersion,
            0x07 => Self::Poll,
            0x08 => Self::Reject,
            0x09 => Self::Disable,
            0x0a => Self::Enable,
            0x0b => Self::ProgramFirmware,
            0x0c => Self::SerialNumber,
            0x0d => Self::UnitData,
            0x0e => Self::ChannelValueData,
            0x11 => Self::Synchronisation,
            0x17 => Self::LastRejectCode,
            0x18 => Self::Hold,
            0x20 => Self::FirmwareVersion,
            0x21 => Self::DatasetVersion,
            0x23 => Self::SetBarcodeReaderConfiguration,
            0x24 => Self::GetBarcodeReaderConfiguration,
            0x25 => Self::GetBarcodeInhibit,
            0x26 => Self::SetBarcodeInhibit,
            0x27 => Self::GetBarcodeData,
            0x30 => Self::ManufacturerExtension,
            0x3f => Self::Empty,
            0x46 => Self::PayoutByDenomination,
            0x4a => Self::SetGenerator,
            0x4b => Self::SetModulus,
            0x4c => Self::RequestKeyExchange,
            0x52 => Self::SmartEmpty,
            0x54 => Self::ConfigureBezel,
            0x56 => Self::PollWithAck,
            0x57 => Self::EventAck,
            0x60 => Self::SetEncryptionKey,
            0x61 => Self::EncryptionReset,
            0x7e => Self::Encrypted,
            _ => Self::Reserved,
        }
    }
}

impl From<MessageType> for u8 {
    fn from(m: MessageType) -> Self {
        m as u8
    }
}

impl From<&MessageType> for u8 {
    fn from(m: &MessageType) -> Self {
        (*m).into()
    }
}

impl From<MessageType> for &'static str {
    fn from(m: MessageType) -> Self {
        match m {
            MessageType::Reset => "Reset",
            MessageType::SetInhibits => "SetInhibits",
            MessageType::DisplayOn => "DisplayOn",
            MessageType::DisplayOff => "DisplayOff",
            MessageType::SetupRequest => "SetupRequest",
            MessageType::HostProtocolVersion => "HostProtocolVersion",
            MessageType::Poll => "Poll",
            MessageType::Reject => "Reject",
            MessageType::Disable => "Disable",
            MessageType::Enable => "Enable",
            MessageType::ProgramFirmware => "ProgramFirmware",
            MessageType::SerialNumber => "SerialNumber",
            MessageType::UnitData => "UnitData",
            MessageType::ChannelValueData => "ChannelValueData",
            MessageType::Synchronisation => "Synchronisation",
            MessageType::LastRejectCode => "LastRejectCode",
            MessageType::Hold => "Hold",
            MessageType::FirmwareVersion => "FirmwareVersion",
            MessageType::DatasetVersion => "DatasetVersion",
            MessageType::SetBarcodeReaderConfiguration => "SetBarcodeReaderConfiguration",
            MessageType::GetBarcodeReaderConfiguration => "GetBarcodeReaderConfiguration",
            MessageType::GetBarcodeInhibit => "GetBarcodeInhibit",
            MessageType::SetBarcodeInhibit => "SetBarcodeInhibit",
            MessageType::GetBarcodeData => "GetBarcodeData",
            MessageType::ManufacturerExtension => "ManufacturerExtension",
            MessageType::Empty => "Empty",
            MessageType::PayoutByDenomination => "PayoutByDenomination",
            MessageType::SetGenerator => "SetGenerator",
            MessageType::SetModulus => "SetModulus",
            MessageType::RequestKeyExchange => "RequestKeyExchange",
            MessageType::SmartEmpty => "SmartEmpty",
            MessageType::ConfigureBezel => "ConfigureBezel",
            MessageType::PollWithAck => "PollWithAck",
            MessageType::EventAck => "EventAck",
            MessageType::SetEncryptionKey => "SetEncryptionKey",
            MessageType::EncryptionReset => "EncryptionReset",
            MessageType::Encrypted => "Encrypted",
            MessageType::Reserved => "Reserved",
        }
    }
}

impl From<&MessageType> for &'static str {
    fn from(m: &MessageType) -> Self {
        (*m).into()
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl_default!(MessageType);
