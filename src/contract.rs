use cosmwasm_std::{
    debug_print, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, to_binary,
    StdError, StdResult, Storage, HumanAddr, log, Uint128
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::utils::{HandleCallback}; //pad_handle_result, pad_query_result, Query,  

use crate::msg::{HandleMsg, QueryMsg, InitMsg, QueryResponse};
use crate::state::{config, config_read, State}; 

const BLOCK_SIZE: usize = 256;


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        rng_addr: deps.api.canonical_address(&HumanAddr(msg.rng_addr))?,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}


/////////////////////////////////////////////////////////////////////////////////
// Enums for callback
/////////////////////////////////////////////////////////////////////////////////

// Calling handle in another contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackRnMsg {
    CallbackRn {entropy: String, cb_msg: Binary, callback_code_hash: String, contract_addr: String},
}

impl HandleCallback for CallbackRnMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}



/////////////////////////////////////////////////////////////////////////////////
// Handles
/////////////////////////////////////////////////////////////////////////////////
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CallRn {entropy, cb_msg, rng_hash, rng_addr} => try_call_rn(deps, env, entropy, cb_msg, rng_hash, rng_addr),
        HandleMsg::ReceiveRn {rn, cb_msg } => try_receive_rn(deps, env, rn, cb_msg),
    }
}

pub fn try_call_rn<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    env: Env,
    entropy: String,
    cb_msg: Binary,
    rng_hash: String,
    rng_addr: String,
) -> StdResult<HandleResponse> {
    let callback_rn_msg = CallbackRnMsg::CallbackRn {
        entropy: entropy,
        cb_msg: cb_msg,
        callback_code_hash: env.contract_code_hash.to_string(),  // to_string() necessary?
        contract_addr: env.contract.address.to_string(),
    };

    let cosmos_msg = callback_rn_msg.to_cosmos_msg(
        rng_hash,
        HumanAddr(rng_addr),
        Some(Uint128(100_000))  // assuming min fees of 100_000 uscrt
    )?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data: None
    })

    // Ok(HandleResponse::default())
}

pub fn try_receive_rn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    rn: [u8; 32],
    cb_msg: Binary,
) -> StdResult<HandleResponse> {
    let config: State = config_read(&deps.storage).load()?;
    let apprv_sender = config.rng_addr;
    // let apprv_sender = deps.api.canonical_address(&env.contract.address)?;  //<-- for user contract, set to scrt-rng's contract addr
    let sender = deps.api.canonical_address(&env.message.sender)?;
    if apprv_sender != sender {
        return Err(StdError::generic_err(
            "receive_rn did not approve sender address",
        ));
    }
    
    let consumer_output = format!("Original message: {:?}, combined with rn: {:?}", 
    String::from_utf8(cb_msg.as_slice().to_vec()),   // <-- will only display properly if the cb_msg input is a String
    rn);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("output", consumer_output)],
        data: None,
    })
    // Ok(HandleResponse::default())
}


/////////////////////////////////////////////////////////////////////////////////
// Queries
/////////////////////////////////////////////////////////////////////////////////
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRngAddr {} => to_binary(&try_get_rng_addr(deps)?),
    }
}

fn try_get_rng_addr<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(QueryResponse {rng_addr: deps.api.human_address(&state.rng_addr)?})
}

