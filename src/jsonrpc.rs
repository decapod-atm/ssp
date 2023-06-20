//! JSON-RPC functionality for the SSP/eSSP server.

use alloc::string::String;

use crate::std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "std")]
mod client;

mod request;
mod response;

#[cfg(feature = "std")]
pub use client::*;

pub use request::*;
pub use response::*;

pub const JSONRPC_ENV_SOCK: &str = "JSONRPC_SOCKET";
pub const JSONRPC_SOCKET_PATH: &str = "/tmp/ssp-jsonrpc.sock";

/// Gets the socket path specified by the environment variable `env`.
///
/// Returns the `default` path if the variable is unset, or another error occurs.
#[cfg(feature = "std")]
pub fn get_socket_path(env: &str, default: &str) -> String {
    std::env::var(env).unwrap_or(default.into())
}

/// Gets the socket path specified by the environment variable `env`.
///
/// Returns the `default` path if the variable is unset, or another error occurs.
#[cfg(not(feature = "std"))]
pub fn get_socket_path(_env: &str, default: &str) -> String {
    default.into()
}

static ID: AtomicU64 = AtomicU64::new(1);

/// Gets the global JSON-RPC ID.
///
/// IDs must be kept in sync between client and server.
pub fn jsonrpc_id() -> u64 {
    ID.load(Ordering::Relaxed)
}

/// Sets the global JSON-RPC ID.
///
/// IDs must be kept in sync between client and server.
///
/// Returns the previously set ID.
pub fn set_jsonrpc_id(id: u64) -> u64 {
    let last = jsonrpc_id();
    ID.store(id, Ordering::SeqCst);
    last
}

/// Sets the globa JSON-RPC ID from a `serde_json::Value`.
///
/// Return the parsed value, if it can be converted to a `u64`, `None` otherwise.
pub fn set_jsonrpc_id_from_val(val: &serde_json::Value) -> Option<u64> {
    if val.is_number() {
        if let Some(v) = val.as_u64() {
            set_jsonrpc_id(v);
            Some(v)
        } else if let Some(v) = val.as_i64() {
            let v = v as u64;
            set_jsonrpc_id(v);
            Some(v)
        } else if let Some(v) = val.as_f64() {
            let v = v as u64;
            set_jsonrpc_id(v);
            Some(v)
        } else {
            None
        }
    } else {
        None
    }
}
