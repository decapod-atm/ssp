use crate::std::{self, fmt, str::FromStr};
use crate::{impl_default, Error, ResponseStatus, Result};

/// Methods for [Event]s.
///
/// Cloned from their corresponding [ResponseStatus]es.
///
/// When adding a new [Event], please add an additional entry here.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Method {
    /// Disable the device, stop accepting notes and commands.
    Disable,
    /// Alias for Disable.
    Stop,
    /// Enable the device to begin accepting notes and commands.
    Enable,
    /// Alias for Enable.
    Accept,
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
            Self::Accept => "accept",
            Self::Disable => "disable",
            Self::Stop => "stop",
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
            "accept" => Self::Accept,
            "disable" => Self::Disable,
            "stop" => Self::Stop,
            "reject" => Self::Reject,
            "stack" => Self::Stack,
            "shutdown" => Self::Shutdown,
            "cashbox_removed" => Self::CashboxRemoved,
            "cashbox_replaced" => Self::CashboxReplaced,
            "disabled" => Self::Disabled,
            "fraud_attempt" => Self::FraudAttempt,
            "note_cleared_return" | "note_cleared_from_front" => Self::NoteClearedFromFront,
            "note_cleared_stack" | "note_cleared_into_cashbox" => Self::NoteClearedIntoCashbox,
            "note_credit" => Self::NoteCredit,
            "cash_insertion" | "read" => Self::Read,
            "rejected" => Self::Rejected,
            "rejecting" => Self::Rejecting,
            "reset" => Self::Reset,
            "stacked" => Self::Stacked,
            "stacker_full" => Self::StackerFull,
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

impl serde::Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Disable => serializer.serialize_unit_variant("Method", 0, "disable"),
            Self::Stop => serializer.serialize_unit_variant("Method", 1, "stop"),
            Self::Enable => serializer.serialize_unit_variant("Method", 2, "enable"),
            Self::Accept => serializer.serialize_unit_variant("Method", 3, "accept"),
            Self::Reject => serializer.serialize_unit_variant("Method", 4, "reject"),
            Self::Stack => serializer.serialize_unit_variant("Method", 5, "stack"),
            Self::Status => serializer.serialize_unit_variant("Method", 6, "status"),
            Self::Shutdown => serializer.serialize_unit_variant("Method", 7, "shutdown"),
            Self::CashboxRemoved => {
                serializer.serialize_unit_variant("Method", 8, "cashbox_removed")
            }
            Self::CashboxReplaced => {
                serializer.serialize_unit_variant("Method", 9, "cashbox_replaced")
            }
            Self::Disabled => serializer.serialize_unit_variant("Method", 10, "disabled"),
            Self::FraudAttempt => serializer.serialize_unit_variant("Method", 11, "fraud_attempt"),
            Self::NoteClearedFromFront => {
                serializer.serialize_unit_variant("Method", 12, "note_cleared_return")
            }
            Self::NoteClearedIntoCashbox => {
                serializer.serialize_unit_variant("Method", 13, "note_cleared_stack")
            }
            Self::NoteCredit => serializer.serialize_unit_variant("Method", 14, "note_credit"),
            Self::Read => serializer.serialize_unit_variant("Method", 15, "cash_insertion"),
            Self::Rejected => serializer.serialize_unit_variant("Method", 16, "rejected"),
            Self::Rejecting => serializer.serialize_unit_variant("Method", 17, "rejecting"),
            Self::Reset => serializer.serialize_unit_variant("Method", 18, "reset"),
            Self::Stacked => serializer.serialize_unit_variant("Method", 19, "stacked"),
            Self::StackerFull => serializer.serialize_unit_variant("Method", 20, "stacker_full"),
            Self::Stacking => serializer.serialize_unit_variant("Method", 21, "stacking"),
            Self::UnsafeJam => serializer.serialize_unit_variant("Method", 22, "unsafe_jam"),
            Self::Fail => serializer.serialize_unit_variant("Method", 23, "fail"),
            Self::Reserved(_) => serializer.serialize_unit_variant("Method", 0xff, "reserved"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Method, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct MethodVisitor;

        impl<'de> serde::de::Visitor<'de> for MethodVisitor {
            type Value = Method;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("`disable` `stop` `enable` `accept` `reject` `stack` `status` `shutdown` `cashbox_removed` `cashbox_replaced` `disabled` `fraud_attempt` `note_cleared_from_front` `note_cleared_return` `note_cleared_into_cashbox` `note_cleared_stack` `note_credit` `read` `rejected` `rejecting` `reset` `stacked` `stacker_full` `stacking` `unsafe_jam` `fail` `reserved`")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Method::from(value.to_lowercase().as_str()))
            }
        }

        deserializer.deserialize_identifier(MethodVisitor)
    }
}

impl_default!(Method);

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "jsonrpc")]
    #[test]
    fn test_method_serialize() -> Result<()> {
        assert_eq!(
            serde_json::to_string(&Method::Disable)?.as_str(),
            "\"disable\""
        );
        assert_eq!(serde_json::to_string(&Method::Stop)?.as_str(), "\"stop\"");
        assert_eq!(
            serde_json::to_string(&Method::Enable)?.as_str(),
            "\"enable\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Accept)?.as_str(),
            "\"accept\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Reject)?.as_str(),
            "\"reject\""
        );
        assert_eq!(serde_json::to_string(&Method::Stack)?.as_str(), "\"stack\"");
        assert_eq!(
            serde_json::to_string(&Method::Status)?.as_str(),
            "\"status\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Shutdown)?.as_str(),
            "\"shutdown\""
        );
        assert_eq!(
            serde_json::to_string(&Method::CashboxRemoved)?.as_str(),
            "\"cashbox_removed\""
        );
        assert_eq!(
            serde_json::to_string(&Method::CashboxReplaced)?.as_str(),
            "\"cashbox_replaced\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Disabled)?.as_str(),
            "\"disabled\""
        );
        assert_eq!(
            serde_json::to_string(&Method::FraudAttempt)?.as_str(),
            "\"fraud_attempt\""
        );
        assert_eq!(
            serde_json::to_string(&Method::NoteClearedFromFront)?.as_str(),
            "\"note_cleared_return\""
        );
        assert_eq!(
            serde_json::to_string(&Method::NoteClearedIntoCashbox)?.as_str(),
            "\"note_cleared_stack\""
        );
        assert_eq!(
            serde_json::to_string(&Method::NoteCredit)?.as_str(),
            "\"note_credit\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Read)?.as_str(),
            "\"cash_insertion\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Rejected)?.as_str(),
            "\"rejected\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Rejecting)?.as_str(),
            "\"rejecting\""
        );
        assert_eq!(serde_json::to_string(&Method::Reset)?.as_str(), "\"reset\"");
        assert_eq!(
            serde_json::to_string(&Method::Stacked)?.as_str(),
            "\"stacked\""
        );
        assert_eq!(
            serde_json::to_string(&Method::StackerFull)?.as_str(),
            "\"stacker_full\""
        );
        assert_eq!(
            serde_json::to_string(&Method::Stacking)?.as_str(),
            "\"stacking\""
        );
        assert_eq!(
            serde_json::to_string(&Method::UnsafeJam)?.as_str(),
            "\"unsafe_jam\""
        );
        assert_eq!(serde_json::to_string(&Method::Fail)?.as_str(), "\"fail\"");

        for i in 0..0xff {
            assert_eq!(
                serde_json::to_string(&Method::Reserved(i))?.as_str(),
                "\"reserved\""
            );
        }

        Ok(())
    }

    #[cfg(feature = "jsonrpc")]
    #[test]
    fn test_method_deserialize() -> Result<()> {
        assert_eq!(
            serde_json::from_str::<Method>("\"disable\"")?,
            Method::Disable
        );
        assert_eq!(serde_json::from_str::<Method>("\"stop\"")?, Method::Stop);
        assert_eq!(
            serde_json::from_str::<Method>("\"enable\"")?,
            Method::Enable
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"accept\"")?,
            Method::Accept
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"reject\"")?,
            Method::Reject
        );
        assert_eq!(serde_json::from_str::<Method>("\"stack\"")?, Method::Stack);
        assert_eq!(
            serde_json::from_str::<Method>("\"status\"")?,
            Method::Status
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"shutdown\"")?,
            Method::Shutdown
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"cashbox_removed\"")?,
            Method::CashboxRemoved
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"cashbox_replaced\"")?,
            Method::CashboxReplaced
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"disabled\"")?,
            Method::Disabled
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"fraud_attempt\"")?,
            Method::FraudAttempt
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"note_cleared_from_front\"")?,
            Method::NoteClearedFromFront
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"note_cleared_return\"")?,
            Method::NoteClearedFromFront
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"note_cleared_into_cashbox\"")?,
            Method::NoteClearedIntoCashbox
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"note_cleared_stack\"")?,
            Method::NoteClearedIntoCashbox
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"note_credit\"")?,
            Method::NoteCredit
        );
        assert_eq!(serde_json::from_str::<Method>("\"read\"")?, Method::Read);
        assert_eq!(
            serde_json::from_str::<Method>("\"rejected\"")?,
            Method::Rejected
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"rejecting\"")?,
            Method::Rejecting
        );
        assert_eq!(serde_json::from_str::<Method>("\"reset\"")?, Method::Reset);
        assert_eq!(
            serde_json::from_str::<Method>("\"stacked\"")?,
            Method::Stacked
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"stacker_full\"")?,
            Method::StackerFull
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"stacking\"")?,
            Method::Stacking
        );
        assert_eq!(
            serde_json::from_str::<Method>("\"unsafe_jam\"")?,
            Method::UnsafeJam
        );
        assert_eq!(serde_json::from_str::<Method>("\"fail\"")?, Method::Fail);
        assert_eq!(
            serde_json::from_str::<Method>("\"reserved\"")?,
            Method::Reserved(0xff)
        );

        Ok(())
    }
}
