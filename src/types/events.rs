//! Event types for polling responses.

use crate::{std::fmt, Error, Result, Vec};

mod cashbox_removed;
mod cashbox_replaced;
mod disabled;
mod fraud_attempt;
mod note_cleared_from_front;
mod note_cleared_into_cashbox;
mod note_credit;
mod read;
mod rejected;
mod rejecting;
mod reset;
mod stacked;
mod stacker_full;
mod stacking;
mod unsafe_jam;

pub use cashbox_removed::*;
pub use cashbox_replaced::*;
pub use disabled::*;
pub use fraud_attempt::*;
pub use note_cleared_from_front::*;
pub use note_cleared_into_cashbox::*;
pub use note_credit::*;
pub use read::*;
pub use rejected::*;
pub use rejecting::*;
pub use reset::*;
pub use stacked::*;
pub use stacker_full::*;
pub use stacking::*;
pub use unsafe_jam::*;

/// Represents a generic event from a polling response.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    method: &'static str,
    data: Vec<u8>,
}

impl Event {
    /// Creates a new [Event] from the provided `method` and `data`.
    pub fn new(method: &'static str, data: &[u8]) -> Result<Self> {
        Ok(Self {
            method,
            data: Vec::from_slice(data)?,
        })
    }

    /// Gets the method.
    pub const fn method(&self) -> &str {
        self.method
    }

    /// Sets the method.
    pub fn set_method(&mut self, method: &'static str) {
        self.method = method;
    }

    /// Gets a reference to the data.
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Gets a mutable reference to the data.
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data.as_mut()
    }

    /// Sets the data.
    pub fn set_data(&mut self, data: &[u8]) -> Result<()> {
        let len = data.len();
        let cap = self.data.capacity();

        if len <= cap {
            self.data.clear();
            self.data.extend_from_slice(data).map_err(|err| err.into())
        } else {
            Err(Error::InvalidDataLength((len, cap)))
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = self.method();
        let data = self.data();

        write!(f, "method: {method}, data: [")?;

        for (i, d) in data.iter().enumerate() {
            write!(f, "{d}")?;
            if i < data.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}
