use crate::std::{fmt, str::FromStr};
use crate::{impl_default, Error, ResponseStatus, Result};

/// Methods for [Event]s.
///
/// Cloned from their corresponding [ResponseStatus]es.
///
/// When adding a new [Event], please add an additional entry here.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Method {
    /// Disable the device, stop accepting notes and commands.
    Disable,
    /// Enable the device to begin accepting notes and commands.
    Enable,
    /// Reject a note in escrow.
    Reject,
    /// Stack a note in escrow.
    Stack,
    /// Get the device status.
    Status,
    /// Shutdown the server.
    Shutdown,
    /// Cashbox removed from device.
    CashboxRemoved = ResponseStatus::CashboxRemoved.to_u8(),
    /// Cashbox replaced into device.
    CashboxReplaced = ResponseStatus::CashboxReplaced.to_u8(),
    /// Device disabled.
    Disabled = ResponseStatus::Disabled.to_u8(),
    /// Fraud attempt detected.
    FraudAttempt = ResponseStatus::FraudAttempt.to_u8(),
    /// Note cleared from bezel, returned to customer.
    NoteClearedFromFront = ResponseStatus::NoteClearedFromFront.to_u8(),
    /// Note cleared from bezel, stacked to cashbox.
    NoteClearedIntoCashbox = ResponseStatus::NoteClearedIntoCashbox.to_u8(),
    /// Note value credited to customer.
    NoteCredit = ResponseStatus::NoteCredit.to_u8(),
    /// Note inserted, and read by the device.
    Read = ResponseStatus::Read.to_u8(),
    /// Device rejected a note.
    Rejected = ResponseStatus::Rejected.to_u8(),
    /// Device is rejecting a note.
    Rejecting = ResponseStatus::Rejecting.to_u8(),
    /// Device reset.
    Reset = ResponseStatus::DeviceReset.to_u8(),
    /// Device stacked a note.
    Stacked = ResponseStatus::Stacked.to_u8(),
    /// Device stacker is full.
    StackerFull = ResponseStatus::StackerFull.to_u8(),
    /// Device is stacking a note.
    Stacking = ResponseStatus::Stacking.to_u8(),
    /// Unsafe jam detected.
    UnsafeJam = ResponseStatus::UnsafeJam.to_u8(),
    /// Device failure.
    Fail = ResponseStatus::Fail.to_u8(),
    /// Currently reserved/unsupported method.
    Reserved(u8),
}

impl Method {
    /// Creates a new [Method].
    pub const fn new() -> Self {
        Self::Reserved(0xff)
    }

    /// Converts the [Method] to a string.
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::Reject => "reject",
            Self::Stack => "stack",
            Self::Shutdown => "shutdown",
            Self::CashboxRemoved => "cashbox_removed",
            Self::CashboxReplaced => "cashbox_replaced",
            Self::Disabled => "disabled",
            Self::FraudAttempt => "fraud_attempt",
            Self::NoteClearedFromFront => "note_cleared_return",
            Self::NoteClearedIntoCashbox => "note_cleared_stack",
            Self::NoteCredit => "note_credit",
            Self::Read => "cash_insertion",
            Self::Rejected => "rejected",
            Self::Rejecting => "rejecting",
            Self::Reset => "reset",
            Self::Stacked => "stacked",
            Self::StackerFull => "stacker_full",
            Self::Stacking => "stacking",
            Self::UnsafeJam => "unsafe_jam",
            Self::Fail => "fail",
            Self::Reserved(_) => "reserved",
        }
    }

    /// Converts a [Method] into a `u8`.
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Reserved(m) => *m,
            method => u8::from(method),
        }
    }

    /// Converts a `u8` into a [Method].
    pub const fn from_u8(val: u8) -> Self {
        Self::from_response_status(ResponseStatus::from_u8(val))
    }

    /// Converts a [ResponseStatus] into a [Method].
    pub const fn from_response_status(val: ResponseStatus) -> Self {
        match val {
            ResponseStatus::CashboxRemoved => Self::CashboxRemoved,
            ResponseStatus::CashboxReplaced => Self::CashboxReplaced,
            ResponseStatus::Disabled => Self::Disabled,
            ResponseStatus::FraudAttempt => Self::FraudAttempt,
            ResponseStatus::NoteClearedFromFront => Self::NoteClearedFromFront,
            ResponseStatus::NoteClearedIntoCashbox => Self::NoteClearedIntoCashbox,
            ResponseStatus::NoteCredit => Self::NoteCredit,
            ResponseStatus::Read => Self::Read,
            ResponseStatus::Rejected => Self::Rejected,
            ResponseStatus::Rejecting => Self::Rejecting,
            ResponseStatus::DeviceReset => Self::Reset,
            ResponseStatus::Stacked => Self::Stacked,
            ResponseStatus::StackerFull => Self::StackerFull,
            ResponseStatus::Stacking => Self::Stacking,
            ResponseStatus::UnsafeJam => Self::UnsafeJam,
            ResponseStatus::Fail => Self::Fail,
            status => Self::Reserved(status.to_u8()),
        }
    }

    /// Converts a [ResponseStatus] into a [Method].
    pub fn to_response_status(&self) -> ResponseStatus {
        match self {
            Self::CashboxRemoved => ResponseStatus::CashboxRemoved,
            Self::CashboxReplaced => ResponseStatus::CashboxReplaced,
            Self::Disabled => ResponseStatus::Disabled,
            Self::FraudAttempt => ResponseStatus::FraudAttempt,
            Self::NoteClearedFromFront => ResponseStatus::NoteClearedFromFront,
            Self::NoteClearedIntoCashbox => ResponseStatus::NoteClearedIntoCashbox,
            Self::NoteCredit => ResponseStatus::NoteCredit,
            Self::Read => ResponseStatus::Read,
            Self::Rejected => ResponseStatus::Read,
            Self::Rejecting => ResponseStatus::Rejecting,
            Self::Reset => ResponseStatus::DeviceReset,
            Self::Stacked => ResponseStatus::Stacked,
            Self::StackerFull => ResponseStatus::StackerFull,
            Self::Stacking => ResponseStatus::Stacking,
            Self::UnsafeJam => ResponseStatus::UnsafeJam,
            Self::Fail => ResponseStatus::Fail,
            method => ResponseStatus::Reserved(method.to_u8()),
        }
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> {
        let res = match val {
            "status" => Self::Status,
            "enable" => Self::Enable,
            "disable" => Self::Disable,
            "reject" => Self::Reject,
            "stack" => Self::Stack,
            "shutdown" => Self::Shutdown,
            "cashbox_removed" => Self::CashboxRemoved,
            "cashbox_replaced" => Self::CashboxReplaced,
            "disabled" => Self::Disabled,
            "fraud_attempt" => Self::FraudAttempt,
            "note_cleared_return" => Self::NoteClearedFromFront,
            "note_cleared_stack" => Self::NoteClearedIntoCashbox,
            "note_credit" => Self::NoteCredit,
            "cash_insertion" => Self::Read,
            "rejected" => Self::Rejected,
            "rejecting" => Self::Rejecting,
            "reset" => Self::Reset,
            "stacked" => Self::Stacked,
            "stacking" => Self::Stacking,
            "unsafe_jam" => Self::UnsafeJam,
            "fail" => Self::Fail,
            _ => Self::Reserved(0xff),
        };

        Ok(res)
    }
}

impl From<&Method> for &'static str {
    fn from(val: &Method) -> Self {
        val.to_str()
    }
}

impl From<Method> for &'static str {
    fn from(val: Method) -> Self {
        (&val).into()
    }
}

impl From<&str> for Method {
    fn from(val: &str) -> Self {
        Self::from_str(val).unwrap_or(Self::Reserved(0xff))
    }
}

impl From<&Method> for u8 {
    fn from(val: &Method) -> Self {
        val.to_u8()
    }
}

impl From<Method> for u8 {
    fn from(val: Method) -> Self {
        (&val).into()
    }
}

impl From<u8> for Method {
    fn from(val: u8) -> Self {
        Self::from_u8(val)
    }
}

impl From<&u8> for Method {
    fn from(val: &u8) -> Self {
        (*val).into()
    }
}

impl From<ResponseStatus> for Method {
    fn from(val: ResponseStatus) -> Self {
        Self::from_response_status(val)
    }
}

impl From<&ResponseStatus> for Method {
    fn from(val: &ResponseStatus) -> Self {
        (*val).into()
    }
}

impl From<Method> for ResponseStatus {
    fn from(val: Method) -> Self {
        val.to_response_status()
    }
}

impl From<&Method> for ResponseStatus {
    fn from(val: &Method) -> Self {
        (*val).into()
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl_default!(Method);
