use crate::lifecycle::{init_env, init_state};
use crate::Data;
use canister_tracing_macros::trace;
use ic_cdk::init;
use sns1_airdrop::init::Args;
use tracing::info;

#[init]
#[trace]
fn init(args: Args) {
    canister_logger::init(args.test_mode);

    let env = init_env([0; 32]);

    let data = Data::new(args.user_index_canister_id, args.admins.into_iter().collect(), args.test_mode);

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}
