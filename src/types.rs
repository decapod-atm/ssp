//! Generic message field types used for command and response messages.

use crate::{std::fmt, tuple_struct_ser};

pub(crate) mod barcode;
pub(crate) mod bezel;
pub(crate) mod channel_value;
pub(crate) mod country_code;
pub(crate) mod device_status;
pub(crate) mod encryption;
pub(crate) mod events;
pub(crate) mod inhibit;
pub(crate) mod last_reject_code;
pub(crate) mod message_type;
pub(crate) mod response_status;
pub(crate) mod sequence_id;
pub(crate) mod serial_number;
pub(crate) mod value_multiplier;
pub(crate) mod version;

pub use barcode::*;
pub use bezel::*;
pub use channel_value::*;
pub use country_code::*;
pub use device_status::*;
pub use encryption::*;
pub use events::*;
pub use inhibit::*;
pub use last_reject_code::*;
pub use message_type::*;
pub use response_status::*;
pub use sequence_id::*;
pub use serial_number::*;
pub use value_multiplier::*;
pub use version::*;

tuple_struct_ser!(UnitType, u8, "Note validator type.");

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
