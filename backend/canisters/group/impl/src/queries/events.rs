use crate::{read_state, RuntimeState};
use group_canister::events::{Response::*, *};
use group_chat_core::EventsResult;
use ic_cdk_macros::query;

#[query]
fn events(args: Args) -> Response {
    read_state(|state| events_impl(args, state))
}

fn events_impl(args: Args, state: &RuntimeState) -> Response {
    let caller = state.env.caller();
    let user_id = state.data.lookup_user_id(caller);

    match state.data.chat.events(
        user_id,
        args.thread_root_message_index,
        args.start_index,
        args.ascending,
        args.max_messages,
        args.max_events,
        args.latest_known_update,
    ) {
        EventsResult::Success(response) => Success(response),
        EventsResult::UserNotInGroup => CallerNotInGroup,
        EventsResult::ThreadNotFound => ThreadMessageNotFound,
        EventsResult::ReplicaNotUpToDate(last_updated) => ReplicaNotUpToDateV2(last_updated),
    }
}
