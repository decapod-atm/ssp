//! JSON-RPC request functionality

use alloc::format;
use smol_jsonrpc::Request;

use super::jsonrpc_id;
use crate::types::events::*;
use crate::{Error, Event, EventPayload, Method};

impl From<&Request> for Event {
    fn from(val: &Request) -> Self {
        let method_str = val.method().unwrap_or("");
        let method = Method::from(method_str.to_lowercase().as_str());

        log::trace!("Method: {method}, method string: {method_str}");

        let payload = match method {
            Method::Fail => {
                let err = match val.params::<Error>() {
                    Ok(err) => err,
                    Err(err) => Error::JsonRpc(format!("failed to parse: {err}")),
                };

                EventPayload::Error(err)
            }
            Method::Disable | Method::Stop | Method::Shutdown => EventPayload::DisableEvent(
                val.params::<DisableEvent>().unwrap_or(DisableEvent::new()),
            ),
            Method::Enable | Method::Accept => EventPayload::EnableEvent(
                val.params::<EnableEvent>()
                    .unwrap_or(EnableEvent::default()),
            ),
            Method::Reject => {
                EventPayload::RejectEvent(val.params::<RejectEvent>().unwrap_or(RejectEvent::new()))
            }
            Method::Stack => EventPayload::StackEvent(
                val.params::<StackEvent>().unwrap_or(StackEvent::default()),
            ),
            Method::Status => EventPayload::StatusEvent(
                val.params::<StatusEvent>()
                    .unwrap_or(StatusEvent::default()),
            ),
            Method::Dispense => EventPayload::DispenseEvent(
                val.params::<DispenseEvent>()
                    .unwrap_or(DispenseEvent::new()),
            ),
            Method::CashboxRemoved => EventPayload::CashboxRemovedEvent(
                val.params::<CashboxRemovedEvent>()
                    .unwrap_or(CashboxRemovedEvent::new()),
            ),
            Method::CashboxReplaced => EventPayload::CashboxReplacedEvent(
                val.params::<CashboxReplacedEvent>()
                    .unwrap_or(CashboxReplacedEvent::new()),
            ),
            Method::Disabled => EventPayload::DisabledEvent(
                val.params::<DisabledEvent>()
                    .unwrap_or(DisabledEvent::new()),
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
                    .unwrap_or(RejectedEvent::new()),
            ),
            Method::Rejecting => EventPayload::RejectingEvent(
                val.params::<RejectingEvent>()
                    .unwrap_or(RejectingEvent::new()),
            ),
            Method::Reset => {
                EventPayload::ResetEvent(val.params::<ResetEvent>().unwrap_or(ResetEvent::new()))
            }
            Method::Stacked => EventPayload::StackedEvent(
                val.params::<StackedEvent>().unwrap_or(StackedEvent::new()),
            ),
            Method::StackerFull => EventPayload::StackerFullEvent(
                val.params::<StackerFullEvent>()
                    .unwrap_or(StackerFullEvent::new()),
            ),
            Method::Stacking => EventPayload::StackingEvent(
                val.params::<StackingEvent>()
                    .unwrap_or(StackingEvent::new()),
            ),
            Method::UnsafeJam => EventPayload::UnsafeJamEvent(
                val.params::<UnsafeJamEvent>()
                    .unwrap_or(UnsafeJamEvent::new()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CountryCode, PayoutDenomination, PayoutDenominationList, PayoutVec, Result};

    #[test]
    fn test_deserialize_dispense_request() -> Result<()> {
        let req_str = r#"{"jsonrpc":"2.0","id":1,"method":"DENOMINATION_DISPENSE","params":{"denominations":[{"value":50,"number":1,"currency":"CAD"}]}}"#;

        let request = serde_json::from_str::<Request>(req_str)?;

        let payout_denom = PayoutDenomination::create(1, 50, CountryCode::CAD);
        let payout_list =
            PayoutDenominationList::create(PayoutVec::from_slice([payout_denom].as_ref())?);
        let exp_event = DispenseEvent::create(payout_list);

        assert_eq!(request.params::<DispenseEvent>()?, exp_event);
        assert_eq!(
            Event::from(&request),
            Event::new(Method::Dispense, EventPayload::DispenseEvent(exp_event))
        );

        Ok(())
    }
}
