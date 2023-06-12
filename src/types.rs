//! Generic message field types used for command and response messages.

use crate::{std::fmt, tuple_struct};

mod barcode;
mod bezel;
mod channel_value;
mod country_code;
mod encryption;
mod inhibit;
mod last_reject_code;
mod message_type;
mod response_status;
mod sequence_id;
mod serial_number;
mod value_multiplier;
mod version;

pub use barcode::*;
pub use bezel::*;
pub use channel_value::*;
pub use country_code::*;
pub use encryption::*;
pub use inhibit::*;
pub use last_reject_code::*;
pub use message_type::*;
pub use response_status::*;
pub use sequence_id::*;
pub use serial_number::*;
pub use value_multiplier::*;
pub use version::*;

tuple_struct!(UnitType, u8, "Note validator type.");

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
