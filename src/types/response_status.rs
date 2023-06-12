use crate::{impl_default, make_list, std::fmt};

/// Represents the device's response status byte.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseStatus {
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
    Reserved = 0xff,
}

impl ResponseStatus {
    /// Creates a new [ResponseStatus].
    pub const fn new() -> Self {
        Self::Reserved
    }

    /// Gets whether the [ResponseStatus] is [ResponseStatus::Ok].
    pub fn is_ok(&self) -> bool {
        *self == Self::Ok
    }
}

impl From<u8> for ResponseStatus {
    fn from(b: u8) -> Self {
        match b {
            0xf0 => Self::Ok,
            0xf1 => Self::DeviceReset,
            0xf2 => Self::CommandNotKnown,
            0xf3 => Self::WrongNumberParameters,
            0xf4 => Self::ParameterOutOfRange,
            0xf5 => Self::CommandCannotBeProcessed,
            0xf8 => Self::Fail,
            0xfa => Self::KeyNotSet,
            _ => Self::Reserved,
        }
    }
}

impl From<ResponseStatus> for u8 {
    fn from(m: ResponseStatus) -> Self {
        m as u8
    }
}

impl From<&ResponseStatus> for u8 {
    fn from(m: &ResponseStatus) -> Self {
        (*m).into()
    }
}

impl From<ResponseStatus> for &'static str {
    fn from(r: ResponseStatus) -> Self {
        match r {
            ResponseStatus::Ok => "OK",
            ResponseStatus::DeviceReset => "DeviceReset",
            ResponseStatus::CommandNotKnown => "CommandNotKnown",
            ResponseStatus::WrongNumberParameters => "WrongNumberParameters",
            ResponseStatus::ParameterOutOfRange => "ParameterOutOfRange",
            ResponseStatus::CommandCannotBeProcessed => "CommandCannotBeProcessed",
            ResponseStatus::Fail => "Fail",
            ResponseStatus::KeyNotSet => "KeyNotSet",
            ResponseStatus::Reserved => "Reserved",
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
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl_default!(ResponseStatus);
make_list!(
    ResponseStatusList,
    ResponseStatus,
    "A list container for [ResponseStatus]s."
);
