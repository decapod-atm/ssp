use super::CountryCode;
use crate::{len::PAYOUT_BLOCK, std::fmt, Error, Result};

mod list;
pub use list::*;

/// Convenience alias for a serialized [PayoutDenomination] array.
pub type PayoutBlock = [u8; PAYOUT_BLOCK];

/// Represents an option byte at the end of the
/// [PayoutByDenominationCommand](crate::PayoutByDenominationCommand) array.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PayoutOption {
    /// This option allows a pre-test of the ability to payout
    /// the requested levels before actual payout executes.
    TestPayoutAmount = 0x19,
    /// Payout the requested amount.
    #[default]
    PayoutAmount = 0x58,
}

impl From<PayoutOption> for u8 {
    fn from(val: PayoutOption) -> Self {
        val as u8
    }
}

impl From<&PayoutOption> for u8 {
    fn from(val: &PayoutOption) -> Self {
        (*val).into()
    }
}

impl From<u8> for PayoutOption {
    fn from(val: u8) -> Self {
        match val {
            0x19 => Self::TestPayoutAmount,
            0x58 => Self::PayoutAmount,
            _ => Self::PayoutAmount,
        }
    }
}

impl From<&PayoutOption> for &'static str {
    fn from(val: &PayoutOption) -> Self {
        match val {
            PayoutOption::TestPayoutAmount => "TestPayoutAmount",
            PayoutOption::PayoutAmount => "PayoutAmount",
        }
    }
}

impl From<PayoutOption> for &'static str {
    fn from(val: PayoutOption) -> Self {
        (&val).into()
    }
}

impl fmt::Display for PayoutOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Represents a payout denomination request block for a
/// [PayoutByDenominationCommand](crate::payout_by_denomination::PayoutByDenomination) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PayoutDenomination {
    number: u16,
    value: u32,
    currency: CountryCode,
}

impl PayoutDenomination {
    /// Creates a new [PayoutDenomination].
    pub const fn new() -> Self {
        Self {
            number: 0,
            value: 0,
            currency: CountryCode::new(),
        }
    }

    /// Creates a new [PayoutDenomination] with the provided parameters.
    pub const fn create(number: u16, value: u32, currency: CountryCode) -> Self {
        Self {
            number,
            value,
            currency,
        }
    }

    /// Gets the number of notes to payout.
    pub const fn number(&self) -> u16 {
        self.number
    }

    /// Sets the number of notes to payout.
    pub fn set_number(&mut self, val: u16) {
        self.number = val;
    }

    /// Builder function that sets the number of notes to payout.
    pub fn with_number(mut self, val: u16) -> Self {
        self.set_number(val);
        self
    }

    /// Gets the denomination value to payout.
    pub const fn value(&self) -> u32 {
        self.value
    }

    /// Sets the denomination value to payout.
    pub fn set_value(&mut self, val: u32) {
        self.value = val;
    }

    /// Builder function that sets the denomination value to payout.
    pub fn with_value(mut self, val: u32) -> Self {
        self.set_value(val);
        self
    }

    /// Gets the denomination [CountryCode].
    pub const fn currency(&self) -> CountryCode {
        self.currency
    }

    /// Sets the denomination [CountryCode].
    pub fn set_currency(&mut self, val: CountryCode) {
        self.currency = val;
    }

    /// Builder function that sets the denomination [CountryCode].
    pub fn with_currency(mut self, val: CountryCode) -> Self {
        self.set_currency(val);
        self
    }

    /// Serializes the [PayoutDenomination] to a mutable buffer.
    ///
    /// Parameters:
    ///
    /// - `buf`: mutable reference to a byte buffer
    ///
    /// Returns:
    ///
    /// - `Ok(())`
    /// - [`Error::InvalidLength`] if `buf` length is less than [`PAYOUT_BLOCK`]
    pub fn to_buffer(&self, buf: &mut [u8]) -> Result<()> {
        let buf_len = buf.len();
        if buf_len < PAYOUT_BLOCK {
            Err(Error::InvalidLength((buf_len, PAYOUT_BLOCK)))
        } else {
            buf[..2].copy_from_slice(self.number().to_le_bytes().as_ref());
            buf[2..6].copy_from_slice(self.value().to_le_bytes().as_ref());
            buf[6..PAYOUT_BLOCK].copy_from_slice(<&str>::from(self.currency()).as_bytes());

            Ok(())
        }
    }

    /// Deserializes the [PayoutDenomination] from a buffer.
    ///
    /// Parameters:
    ///
    /// - `buf`: reference to a byte buffer
    ///
    /// Returns:
    ///
    /// - Ok([`PayoutDenomination`])
    /// - [`Error::InvalidLength`] if `buf` length is less than [`PAYOUT_BLOCK`]
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        let len = buf.len();

        if len < PAYOUT_BLOCK {
            Err(Error::InvalidLength((len, PAYOUT_BLOCK)))
        } else {
            let number = u16::from_le_bytes(buf[..2].try_into().unwrap_or([0; 2]));
            let value = u32::from_le_bytes(buf[2..6].try_into().unwrap_or([0; 4]));
            let currency = CountryCode::from(&buf[6..PAYOUT_BLOCK]);

            Ok(Self::create(number, value, currency))
        }
    }
}

impl From<PayoutDenomination> for PayoutBlock {
    fn from(val: PayoutDenomination) -> Self {
        let mut res = [0u8; PAYOUT_BLOCK];

        // buffer is a valid length, so `to_buffer` will never fail.
        val.to_buffer(res.as_mut()).ok();

        res
    }
}

impl From<&PayoutBlock> for PayoutDenomination {
    fn from(val: &PayoutBlock) -> Self {
        // the unwrap branch should never happen, since the buffer length is valid.
        // just in case... use a safe backup
        Self::from_buffer(val.as_ref()).unwrap_or(Self::new())
    }
}

impl TryFrom<&[u8]> for PayoutDenomination {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::from_buffer(val)
    }
}

impl fmt::Display for PayoutDenomination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""number":{},"#, self.number())?;
        write!(f, r#""value":{},"#, self.value())?;
        write!(f, r#""currency":{}"#, self.currency())?;
        write!(f, "}}")
    }
}
