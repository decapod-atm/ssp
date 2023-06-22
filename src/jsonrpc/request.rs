//! JSON-RPC request functionality

use alloc::format;
use smol_jsonrpc::Request;

use super::jsonrpc_id;
use crate::types::events::*;
use crate::{Error, Event, EventPayload, Method};

impl From<&Request> for Event {
    fn from(val: &Request) -> Self {
        let method = Method::from(val.method().unwrap_or(""));

        let payload = match method {
            Method::Fail => {
                let err = match val.params::<Error>() {
                    Ok(err) => err,
                    Err(err) => Error::JsonRpc(format!("failed to parse: {err}")),
                };

                EventPayload::Error(err)
            }
            Method::Disable | Method::Stop | Method::Shutdown => EventPayload::DisableEvent(
                val.params::<DisableEvent>()
                    .unwrap_or(DisableEvent::default()),
            ),
            Method::Enable | Method::Accept => EventPayload::EnableEvent(
                val.params::<EnableEvent>()
                    .unwrap_or(EnableEvent::default()),
            ),
            Method::Reject => EventPayload::RejectEvent(
                val.params::<RejectEvent>()
                    .unwrap_or(RejectEvent::default()),
            ),
            Method::Stack => EventPayload::StackEvent(
                val.params::<StackEvent>().unwrap_or(StackEvent::default()),
            ),
            Method::Status => EventPayload::StatusEvent(
                val.params::<StatusEvent>()
                    .unwrap_or(StatusEvent::default()),
            ),
            Method::CashboxRemoved => EventPayload::CashboxRemovedEvent(
                val.params::<CashboxRemovedEvent>()
                    .unwrap_or(CashboxRemovedEvent::default()),
            ),
            Method::CashboxReplaced => EventPayload::CashboxReplacedEvent(
                val.params::<CashboxReplacedEvent>()
                    .unwrap_or(CashboxReplacedEvent::default()),
            ),
            Method::Disabled => EventPayload::DisabledEvent(
                val.params::<DisabledEvent>()
                    .unwrap_or(DisabledEvent::default()),
            ),
            Method::FraudAttempt => EventPayload::FraudAttemptEvent(
                val.params::<FraudAttemptEvent>()
                    .unwrap_or(FraudAttemptEvent::default()),
            ),
            Method::NoteClearedFromFront => EventPayload::NoteClearedFromFrontEvent(
                val.params::<NoteClearedFromFrontEvent>()
                    .unwrap_or(NoteClearedFromFrontEvent::default()),
            ),
            Method::NoteClearedIntoCashbox => EventPayload::NoteClearedIntoCashboxEvent(
                val.params::<NoteClearedIntoCashboxEvent>()
                    .unwrap_or(NoteClearedIntoCashboxEvent::default()),
            ),
            Method::NoteCredit => EventPayload::NoteCreditEvent(
                val.params::<NoteCreditEvent>()
                    .unwrap_or(NoteCreditEvent::default()),
            ),
            Method::Read => {
                EventPayload::ReadEvent(val.params::<ReadEvent>().unwrap_or(ReadEvent::default()))
            }
            Method::Rejected => EventPayload::RejectedEvent(
                val.params::<RejectedEvent>()
                    .unwrap_or(RejectedEvent::default()),
            ),
            Method::Rejecting => EventPayload::RejectingEvent(
                val.params::<RejectingEvent>()
                    .unwrap_or(RejectingEvent::default()),
            ),
            Method::Reset => EventPayload::ResetEvent(
                val.params::<ResetEvent>().unwrap_or(ResetEvent::default()),
            ),
            Method::Stacked => EventPayload::StackedEvent(
                val.params::<StackedEvent>()
                    .unwrap_or(StackedEvent::default()),
            ),
            Method::StackerFull => EventPayload::StackerFullEvent(
                val.params::<StackerFullEvent>()
                    .unwrap_or(StackerFullEvent::default()),
            ),
            Method::Stacking => EventPayload::StackingEvent(
                val.params::<StackingEvent>()
                    .unwrap_or(StackingEvent::default()),
            ),
            Method::UnsafeJam => EventPayload::UnsafeJamEvent(
                val.params::<UnsafeJamEvent>()
                    .unwrap_or(UnsafeJamEvent::default()),
            ),
            Method::Reserved(m) => {
                EventPayload::Error(Error::JsonRpc(format!("reserved method: {m}")))
            }
        };

        Self::new(method, payload)
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
            .with_params(val.payload().to_json())
    }
}

impl From<Event> for Request {
    fn from(val: Event) -> Self {
        (&val).into()
    }
}
