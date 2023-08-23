use crate::{len, std::fmt, CommandOps, MessageOps, MessageType};

mod index {
    pub const PAYOUT_OPTION: usize = 4;
}

mod bitmask {
    pub const OPTION: u8 = 0b11;
}

bitfield! {
    /// Option byte for the [EnablePayoutCommand](crate::EnablePayoutCommand) message.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct EnablePayoutOption(u8);
    u8;
    /// Option bitfield zero, has different meaning based on device type.
    ///
    /// - `nv11`: `GIVE_VALUE_ON_STORED` - set to enable the value of the note stored to be given
    /// with the `Note Stored` event.
    /// - `smart payout`: `REQUIRE_FULL_STARTUP` - if set, the `Smart Payout` will return busy
    /// until it has fully completed the startup procedure.
    pub field_0, set_field_0: 0;
    /// Option bitfield one, has different meaning based on device type.
    ///
    /// - `nv11`: `NO_HOLD_NOTE_ON_PAYOUT` - set to enable the function of fully rejecting the
    /// dispensed banknote rather then holding it in the bezel.
    /// - `smart payout`: `OPTIMISE_FOR_PAYIN_SPEED` - if set, the `Smart Payout` will always move
    /// towards an empty slot when idle to try and ensure the shortest pay in speed possible.
    pub field_1, set_field_1: 1;
}

impl From<u8> for EnablePayoutOption {
    fn from(val: u8) -> Self {
        Self(val & bitmask::OPTION)
    }
}

impl From<&EnablePayoutOption> for u8 {
    fn from(val: &EnablePayoutOption) -> Self {
        val.0
    }
}

impl From<EnablePayoutOption> for u8 {
    fn from(val: EnablePayoutOption) -> Self {
        (&val).into()
    }
}

impl fmt::Display for EnablePayoutOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f0 = self.field_0();
        let f1 = self.field_1();

        write!(f, r#"{{"field_0": {f0}, "field_1": {f1}}}"#)
    }
}

/// EnablePayout - Command (0x5C)
///
/// A command to enable the attached payout device for storing/paying out notes.
///
/// A successful enable will return OK.
///
/// If there is a problem the reply will be generic response `COMMAND_CANNOT_BE_PROCESSED`, followed by an error code.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnablePayoutCommand {
    buf: [u8; len::ENABLE_PAYOUT_COMMAND],
}

impl EnablePayoutCommand {
    /// Creates a new [EnablePayoutCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENABLE_PAYOUT_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::EnablePayout);

        msg
    }

    /// Gets the [EnablePayoutOption].
    pub fn option(&self) -> EnablePayoutOption {
        self.buf[index::PAYOUT_OPTION].into()
    }

    /// Sets the [EnablePayoutOption].
    pub fn set_option(&mut self, option: EnablePayoutOption) {
        self.buf[index::PAYOUT_OPTION] = option.into();
    }

    /// Builder function that sets the [EnablePayoutOption].
    pub fn with_option(mut self, option: EnablePayoutOption) -> Self {
        self.set_option(option);
        self
    }
}

impl_default!(EnablePayoutCommand);
impl_message_from_buf!(EnablePayoutCommand);
impl_message_ops!(EnablePayoutCommand);
impl_command_ops!(EnablePayoutCommand);

impl fmt::Display for EnablePayoutCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""stx": "0x{:02x}", "#, self.stx())?;
        write!(f, r#""sequence_id": {}, "#, self.sequence_id())?;
        write!(f, r#""len": "0x{:02x}", "#, self.data_len())?;
        write!(f, r#""command": {}, "#, self.command())?;
        write!(f, r#""option": {}, "#, self.option())?;
        write!(f, r#""checksum": "0x{:04x}""#, self.checksum())?;
        write!(f, "}}")
    }
}
