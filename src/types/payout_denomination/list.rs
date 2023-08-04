use super::PayoutDenomination;
use crate::{
    arrays::{deserialize_vec, serialize_vec},
    std::{cmp, fmt, slice},
};

/// Maximum number of payout requests in a single
/// [PayoutByDenominationCommand](crate::PayoutByDenomination) message.
pub const MAX_PAYOUTS: usize = 20;

/// Convenience alias for a [PayoutDenomination] vector.
pub type PayoutVec = heapless::Vec<PayoutDenomination, MAX_PAYOUTS>;

/// Container for a list of [PayoutDenomination]s.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PayoutDenominationList {
    #[serde(serialize_with = "serialize_vec", deserialize_with = "deserialize_vec")]
    denominations: PayoutVec,
}

impl PayoutDenominationList {
    /// Creates a new empty list.
    pub fn new() -> Self {
        Self {
            denominations: PayoutVec::new(),
        }
    }

    /// Gets an iterator over the list.
    pub fn iter(&self) -> slice::Iter<PayoutDenomination> {
        self.denominations.iter()
    }

    /// Gets a mutable iterator over the list.
    pub fn iter_mut(&mut self) -> slice::IterMut<PayoutDenomination> {
        self.denominations.iter_mut()
    }

    /// Gets the list length.
    pub fn len(&self) -> usize {
        self.denominations.len()
    }

    /// Gets the list capacity.
    pub fn capacity(&self) -> usize {
        self.denominations.capacity()
    }

    /// Gets whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.denominations.is_empty()
    }

    /// Get the list as a reference to its inner container type.
    pub fn as_inner(&self) -> &PayoutVec {
        &self.denominations
    }

    /// Get the list as a mutable reference to its inner container type.
    pub fn as_inner_mut(&mut self) -> &mut PayoutVec {
        &mut self.denominations
    }

    /// Converts the list into its inner container type.
    pub fn into_inner(self) -> PayoutVec {
        self.denominations
    }
}

impl AsRef<[PayoutDenomination]> for PayoutDenominationList {
    fn as_ref(&self) -> &[PayoutDenomination] {
        self.denominations.as_slice()
    }
}

impl AsMut<[PayoutDenomination]> for PayoutDenominationList {
    fn as_mut(&mut self) -> &mut [PayoutDenomination] {
        self.denominations.as_mut()
    }
}

impl From<PayoutVec> for PayoutDenominationList {
    fn from(val: PayoutVec) -> Self {
        Self { denominations: val }
    }
}

impl From<&[PayoutDenomination]> for PayoutDenominationList {
    fn from(val: &[PayoutDenomination]) -> Self {
        let len = cmp::min(val.len(), MAX_PAYOUTS);
        let mut denominations = PayoutVec::new();

        // the above call to `min` ensures the length is in the valid range
        denominations.extend_from_slice(&val[..len]).ok();

        Self { denominations }
    }
}

impl<const N: usize> From<[PayoutDenomination; N]> for PayoutDenominationList {
    fn from(val: [PayoutDenomination; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<&[PayoutDenomination; N]> for PayoutDenominationList {
    fn from(val: &[PayoutDenomination; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for PayoutDenominationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""denominations":["#)?;

        for (i, denom) in self.iter().enumerate() {
            write!(f, "{denom}")?;

            if i < self.len() - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, "]}}")
    }
}

impl Default for PayoutDenominationList {
    fn default() -> Self {
        Self::new()
    }
}
