use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{CommunityCanisterCommunitySummary, CommunityId, GateCheckFailedReason, VerifiedCredentialGateArgs};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub community_id: CommunityId,
    pub invite_code: Option<u64>,
    pub verified_credential_args: Option<VerifiedCredentialGateArgs>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(Box<CommunityCanisterCommunitySummary>),
    AlreadyInCommunity(Box<CommunityCanisterCommunitySummary>),
    GateCheckFailed(GateCheckFailedReason),
    CommunityNotFound,
    CommunityNotPublic,
    NotInvited,
    MemberLimitReached(u32),
    UserBlocked,
    UserSuspended,
    CommunityFrozen,
    InternalError(String),
}
