use alloc::format;
use smol_jsonrpc::Response;

use super::jsonrpc_id;
use crate::{Error, Event, EventPayload, Method};

impl From<&Event> for Response {
    fn from(val: &Event) -> Self {
        let res = Self::new().with_id(jsonrpc_id());

        match val.payload() {
            EventPayload::Error(err) => res.with_error(err.into()),
            payload => res.with_result(payload),
        }
    }
}

impl From<Event> for Response {
    fn from(val: Event) -> Self {
        (&val).into()
    }
}

impl From<&Response> for Event {
    fn from(val: &Response) -> Self {
        if !val.result_is_null() {
            let (method, payload) = match val.result::<EventPayload>() {
                Ok(evt) => (evt.method(), evt),
                Err(err) => (
                    Method::Fail,
                    EventPayload::Error(Error::JsonRpc(format!("{err}"))),
                ),
            };

            Self::new(method, payload)
        } else if let Some(err) = val.error() {
            Self::new(Method::Fail, EventPayload::Error(err.into()))
        } else {
            Self::from(Method::Fail)
        }
    }
}

impl From<Response> for Event {
    fn from(val: Response) -> Self {
        (&val).into()
    }
}
