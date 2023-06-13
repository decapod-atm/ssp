use crate::{impl_default, make_list, std::fmt};

/// Represents the device's response status byte.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseStatus {
    // Events
    /// Note cleared from front bezel.
    ///
    /// During the device power up sequence a bill was detected as being in the note path. This bill
    /// is then rejected from the device via the bezel and this event is issued. If the bill value is
    /// known then the channel number is given in the data byte, otherwise the data byte will be
    /// zero value.
    NoteClearedFromFront = 0xe1,
    /// Note cleared into the cashbox.
    ///
    /// During the device power up sequence a bill was detected as being in the stack path. This bill
    /// is then moved into the device cashbox and this event is issued. If the bill value is known
    /// then the channel number is given in the data byte, otherwise the data byte will be zero
    /// value.
    NoteClearedIntoCashbox = 0xe2,
    /// Cashbox removed from device.
    ///
    /// The system has detected that the cashbox unit has been removed from it's working position.
    ///
    /// The system will remain disabled for bill entry until the cashbox unit is replaced into it's
    /// working position.
    CashboxRemoved = 0xe3,
    /// Cashbox replaced into the device.
    ///
    /// The device cashbox box unit has been detected as replaced into it's working position.
    ///
    /// The validator will re-enable if it has not already been disabled by the host system.
    CashboxReplaced = 0xe4,
    /// An attempt to defraud the device ocurred.
    ///
    /// The validator system has detected an attempt to mauipulate the coin/banknote in order to
    /// fool the system to register credits with no monies added.
    FraudAttempt = 0xe6,
    /// Stacker unit is full.
    ///
    /// Event in response to poll given when the device has detected that the stacker unit has
    /// stacked it's full limit of banknotes.
    StackerFull = 0xe7,
    /// Device is in a disabled state.
    ///
    /// A disabled event is given in response to a poll command when a device has been disabled by
    /// the host or by some other internal function of the device.
    Disabled = 0xe8,
    /// A bill has been detected as jammed during it's transport through the validator.
    ///
    /// An unsafe jam indicates that this bill may be in a position where the user could retrieve it from the
    /// validator bezel.
    UnsafeJam = 0xe9,
    /// A bill has been transported trough the banknote validator and is in it's stacked position.
    Stacked = 0xeb,
    /// The bill is currently being transported to and through the device stacker.
    Stacking = 0xcc,
    /// A bill has been rejected back to the user by the Banknote Validator.
    Rejected = 0xec,
    /// A bill is in the process of being rejected back to the user by the Banknte Validator.
    Rejecting = 0xed,
    /// Add note credit from escrow to storage.
    ///
    /// This event is generated when the banknote has been moved from the escrow position to a
    /// safe position within the validator system where the banknote cannot be retreived by the user.
    ///
    /// At this point, it is safe for the host to use this event as it's 'Credt' point.
    NoteCredit = 0xee,
    /// An event given when the BNV is reading a banknote.
    ///
    /// If the event data byte is zero, then the note is in the process of being scanned and validated.
    ///
    /// If the data byte value changes from zero to a value greater then zero, this indicates a valid banknote is
    /// now held in the escrow position. The byte value shows the channel of the banknote that has been
    /// validated.
    ///
    /// A poll command after this value has been given will cause the banknote to be accepted from
    /// the escrow position. The host can also issue a reject command at this point to reject the banknote
    /// back to the user.
    Read = 0xef,
    /// OK is the first byte returned in the response to a successful command. It does not indicate
    /// that the command has completed, just that it has been received and understood.
    Ok = 0xf0,
    /// Device reset
    DeviceReset = 0xf1,
    /// Returned when an invalid command is received by a peripheral. Check the firmware is up to
    /// date and the protocol level is set correctly.
    CommandNotKnown = 0xf2,
    /// Indicates the command was received by the device but the parameters provided with the
    /// command did not match what the device was expecting.
    ///
    /// Check the specification to ensure the arguments provided with the command were valid
    /// and that the correct protocol version is being used
    WrongNumberParameters = 0xf3,
    /// Indicates the command was received by the device but the parameters provided with the
    /// command were out of available range. Examples of this are providing a non-prime number
    /// to set generator command (0x4A).
    ParameterOutOfRange = 0xf4,
    /// A command sent could not be processed at that time.
    ///
    /// This response can have an additional byte giving the reason the command cannot be processed.
    /// Check individual device command details for details An example of this is asking a Hopper to
    /// payout whilst it is already dispensing coins.
    ///
    /// Check the poll response for the state of the device and retry the command when the device
    /// is enabled and not busy.
    CommandCannotBeProcessed = 0xf5,
    /// Failure response
    ///
    /// Used if none of the other error conditions are applicable or as detailed in command
    /// documentation. An example is setting protocol version to a number greater than that
    /// supported by the device.
    Fail = 0xf8,
    /// The device is in encrypted communication mode, but the encryption keys have not been
    /// negotiated.
    KeyNotSet = 0xfa,
    /// Reserved for future use
    Reserved(u8),
}

impl ResponseStatus {
    /// Creates a new [ResponseStatus].
    pub const fn new() -> Self {
        Self::Reserved(0xff)
    }

    /// Gets whether the [ResponseStatus] is [ResponseStatus::Ok].
    pub fn is_ok(&self) -> bool {
        *self == Self::Ok
    }
}

impl From<u8> for ResponseStatus {
    fn from(val: u8) -> Self {
        match val {
            0xe1 => Self::NoteClearedFromFront,
            0xe2 => Self::NoteClearedIntoCashbox,
            0xe3 => Self::CashboxRemoved,
            0xe4 => Self::CashboxReplaced,
            0xe6 => Self::FraudAttempt,
            0xe7 => Self::StackerFull,
            0xe8 => Self::Disabled,
            0xe9 => Self::UnsafeJam,
            0xeb => Self::Stacked,
            0xcc => Self::Stacking,
            0xec => Self::Rejected,
            0xed => Self::Rejecting,
            0xee => Self::NoteCredit,
            0xef => Self::Read,
            0xf0 => Self::Ok,
            0xf1 => Self::DeviceReset,
            0xf2 => Self::CommandNotKnown,
            0xf3 => Self::WrongNumberParameters,
            0xf4 => Self::ParameterOutOfRange,
            0xf5 => Self::CommandCannotBeProcessed,
            0xf8 => Self::Fail,
            0xfa => Self::KeyNotSet,
            res => Self::Reserved(res),
        }
    }
}

impl From<ResponseStatus> for u8 {
    fn from(val: ResponseStatus) -> Self {
        match val {
            ResponseStatus::NoteClearedFromFront => 0xe1,
            ResponseStatus::NoteClearedIntoCashbox => 0xe2,
            ResponseStatus::CashboxRemoved => 0xe3,
            ResponseStatus::CashboxReplaced => 0xe4,
            ResponseStatus::FraudAttempt => 0xe6,
            ResponseStatus::StackerFull => 0xe7,
            ResponseStatus::Disabled => 0xe8,
            ResponseStatus::UnsafeJam => 0xe9,
            ResponseStatus::Stacked => 0xeb,
            ResponseStatus::Stacking => 0xcc,
            ResponseStatus::Rejected => 0xec,
            ResponseStatus::Rejecting => 0xed,
            ResponseStatus::NoteCredit => 0xee,
            ResponseStatus::Read => 0xef,
            ResponseStatus::Ok => 0xf0,
            ResponseStatus::DeviceReset => 0xf1,
            ResponseStatus::CommandNotKnown => 0xf2,
            ResponseStatus::WrongNumberParameters => 0xf3,
            ResponseStatus::ParameterOutOfRange => 0xf4,
            ResponseStatus::CommandCannotBeProcessed => 0xf5,
            ResponseStatus::Fail => 0xf8,
            ResponseStatus::KeyNotSet => 0xfa,
            ResponseStatus::Reserved(s) => s,
        }
    }
}

impl From<&ResponseStatus> for u8 {
    fn from(val: &ResponseStatus) -> Self {
        (*val).into()
    }
}

impl From<ResponseStatus> for &'static str {
    fn from(val: ResponseStatus) -> Self {
        match val {
            ResponseStatus::NoteClearedFromFront => "NoteClearedFromFront",
            ResponseStatus::NoteClearedIntoCashbox => "NoteClearedIntoCashbox",
            ResponseStatus::CashboxRemoved => "CashboxRemoved",
            ResponseStatus::CashboxReplaced => "CashboxReplaced",
            ResponseStatus::FraudAttempt => "FraudAttempt",
            ResponseStatus::StackerFull => "StackerFull",
            ResponseStatus::Disabled => "Disabled",
            ResponseStatus::UnsafeJam => "UnsafeJam",
            ResponseStatus::Stacked => "Stacked",
            ResponseStatus::Stacking => "Stacking",
            ResponseStatus::Rejected => "Rejected",
            ResponseStatus::Rejecting => "Rejecting",
            ResponseStatus::NoteCredit => "NoteCredit",
            ResponseStatus::Read => "Read",
            ResponseStatus::Ok => "OK",
            ResponseStatus::DeviceReset => "DeviceReset",
            ResponseStatus::CommandNotKnown => "CommandNotKnown",
            ResponseStatus::WrongNumberParameters => "WrongNumberParameters",
            ResponseStatus::ParameterOutOfRange => "ParameterOutOfRange",
            ResponseStatus::CommandCannotBeProcessed => "CommandCannotBeProcessed",
            ResponseStatus::Fail => "Fail",
            ResponseStatus::KeyNotSet => "KeyNotSet",
            ResponseStatus::Reserved(_) => "Reserved",
        }
    }
}

impl From<&ResponseStatus> for &'static str {
    fn from(r: &ResponseStatus) -> Self {
        (*r).into()
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reserved(s) => write!(f, "Reserved(0x{s:02x})"),
            _ => write!(f, "{}", <&'static str>::from(self)),
        }
    }
}

impl_default!(ResponseStatus);

make_list!(
    ResponseStatusList,
    ResponseStatus,
    "A list container for [ResponseStatus]s."
);
