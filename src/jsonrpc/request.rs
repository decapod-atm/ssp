//! JSON-RPC request functionality

use alloc::format;
use smol_jsonrpc::Request;

use super::jsonrpc_id;
use crate::{Error, Event, EventPayload, Method};

impl From<&Request> for Event {
    fn from(val: &Request) -> Self {
        let method = Method::from(val.method().unwrap_or(""));

        let payload = match val.params::<EventPayload>() {
            Ok(event) => event,
            Err(err) => EventPayload::Error(err.into()),
        };

        let payload_method = payload.method();

        if method == payload_method {
            Self::new(method, payload)
        } else {
            Self::new(
                Method::Fail,
                EventPayload::Error(Error::JsonRpc(format!(
                    "Invalid event, have: {payload_method}, expect: {method}, payload: {payload}"
                ))),
            )
        }
    }
}

impl From<Request> for Event {
    fn from(val: Request) -> Self {
        (&val).into()
    }
}

impl From<&Event> for Request {
    fn from(val: &Event) -> Self {
        Request::new()
            .with_id(jsonrpc_id())
            .with_method(val.method().to_str())
            .with_params(val.payload())
    }
}

impl From<Event> for Request {
    fn from(val: Event) -> Self {
        (&val).into()
    }
}
