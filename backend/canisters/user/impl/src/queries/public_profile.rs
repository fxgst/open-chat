use crate::read_state;
use ic_cdk::query;
use user_canister::public_profile::{Response::*, *};

#[query]
fn public_profile(_args: Args) -> Response {
    read_state(|state| {
        Success(PublicProfile {
            username: state.data.username.value.clone(),
            display_name: state.data.display_name.value.clone(),
            avatar_id: state.data.avatar.value.as_ref().map(|a| a.id),
            bio: state.data.bio.value.clone(),
            is_premium: state.data.phone_is_verified || state.data.storage_limit > 0,
            phone_is_verified: state.data.phone_is_verified,
            created: state.data.user_created,
        })
    })
}
