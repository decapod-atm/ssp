#![cfg_attr(not(feature = "std"), no_std)]

//! This library is an implementation of the Smiley Secure Protocol.
//!
//! From the SSP Implementation Guide:
//!
//! ```nobuild, no_run
//! Smiley Secure Protocol (SSP) is a serial communication protocol designed by Innovative
//! Technology LTD to address problems historically experienced by cash handling systems in
//! gaming machines. Problems such as acceptor swapping, reprogramming acceptors and
//! line tapping.
//! ```
//!
//! The current implementation supports both the standard (SSP), and encrypted variant (eSSP).

#[macro_use(format)]
extern crate alloc;

#[macro_use(bitfield)]
extern crate bitfield;

#[cfg(not(feature = "std"))]
use core as std;
#[cfg(feature = "std")]
#[allow(clippy::single_component_path_imports)]
use std;

mod macros;

pub mod aes;
pub mod arrays;
pub mod channel_value_data;
mod channels;
pub mod configure_bezel;
pub mod crc;
pub mod dataset_version;
pub mod disable;
pub mod display_off;
pub mod display_on;
pub mod empty;
pub mod enable;
pub mod encrypted;
pub mod encryption_reset;
pub mod error;
pub mod event_ack;
pub mod get_barcode_data;
pub mod get_barcode_inhibit;
pub mod get_barcode_reader_configuration;
pub mod hold;
pub mod host_protocol_version;
#[cfg(feature = "jsonrpc")]
pub mod jsonrpc;
pub mod keys;
pub mod last_reject_code;
pub mod len;
pub mod message;
pub mod payout_by_denomination;
pub mod poll;
pub mod poll_with_ack;
pub mod primes;
pub mod reject;
pub mod request_key_exchange;
pub mod reset;
pub mod rng;
pub mod serial_number;
pub mod set_barcode_inhibit;
pub mod set_barcode_reader_configuration;
pub mod set_encryption_key;
pub mod set_generator;
pub mod set_inhibits;
pub mod set_modulus;
pub mod setup_request;
pub mod smart_empty;
pub mod sync;
pub mod types;
pub mod unit_data;

pub use channel_value_data::*;
pub use channels::*;
pub use configure_bezel::*;
pub use dataset_version::*;
pub use disable::*;
pub use display_off::*;
pub use display_on::*;
pub use empty::*;
pub use enable::*;
pub use encrypted::*;
pub use encryption_reset::*;
pub use error::*;
pub use event_ack::*;
pub use get_barcode_data::*;
pub use get_barcode_inhibit::*;
pub use get_barcode_reader_configuration::*;
pub use hold::*;
pub use host_protocol_version::*;
pub use keys::*;
pub use last_reject_code::*;
pub use message::{index as message_index, *};
pub use payout_by_denomination::*;
pub use poll::*;
pub use poll_with_ack::*;
pub use reject::*;
pub use request_key_exchange::*;
pub use reset::*;
pub use rng::*;
pub use serial_number::*;
pub use set_barcode_inhibit::*;
pub use set_barcode_reader_configuration::*;
pub use set_encryption_key::*;
pub use set_generator::*;
pub use set_inhibits::*;
pub use set_modulus::*;
pub use setup_request::*;
pub use smart_empty::*;
pub use sync::*;
pub use types::*;
pub use unit_data::*;

pub type Vec<T> = heapless::Vec<T, { len::MAX_DATA }>;
