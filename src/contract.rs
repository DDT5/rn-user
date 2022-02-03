use cosmwasm_std::{
    debug_print, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, to_binary,
    StdError, StdResult, Storage, HumanAddr, log, //Uint128, // CanonicalAddr, 
};

use crate::msg::{
    HandleMsg, QueryMsg, InitMsg, QueryResponse, CallbackRnMsg, CreateRnMsg, FulfillRnMsg, GenerateViewingKeyMsg,
    QueryRnMsg, QueryAnswerMsg,
};
use crate::state::{config, config_read, State, config_vk, config_read_vk, VkStore}; 

use crate::viewing_key::{ViewingKey}; 
use secret_toolkit::utils::{HandleCallback, Query}; //pad_handle_result, pad_query_result, 

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        rng_hash: msg.rng_hash,
        rng_addr: deps.api.canonical_address(&HumanAddr(msg.rng_addr))?,
        // rng_interf_addr: deps.api.canonical_address(&HumanAddr(msg.rng_interf_addr))?,
    };
    config(&mut deps.storage).save(&state)?;
    
    let vk = VkStore {
        vks: vec![],
    };
    config_vk(&mut deps.storage).save(&vk)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
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
        HandleMsg::ReceiveFRn {rn, cb_msg, purpose} => try_receive_fulfill_rn(deps, env, rn, cb_msg, purpose),
        HandleMsg::TriggerCreateRn {
            entropy, cb_msg, receiver_code_hash, receiver_addr, purpose, max_blk_delay, rng_hash, rng_addr,
        } => try_trigger_create_rn (deps, env, entropy, cb_msg, receiver_code_hash, receiver_addr, purpose, max_blk_delay, rng_hash, rng_addr),
        HandleMsg::TriggerFulfillRn {creator_addr, receiver_code_hash, purpose, rng_hash, rng_addr
        } => try_trigger_fulfill_rn(deps, env, creator_addr, receiver_code_hash, purpose, rng_hash, rng_addr),
        HandleMsg::TriggerGenerateVk {receiver_code_hash, rng_hash, rng_addr,
        } => try_trigger_generate_vk(deps, env, receiver_code_hash, rng_hash, rng_addr),
        HandleMsg::ReceiveViewingKey {key,} => try_receive_viewing_key(deps, env, key),
        HandleMsg::TriggerQueryRn {entropy, optionalvk} => try_trigger_query_rn(deps, env, entropy, optionalvk),
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
        None // Some(Uint128(100_000)) 
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
    // let apprv_sender = config.rng_addr;
    // let apprv_sender = deps.api.canonical_address(&env.contract.address)?;  //<-- for user contract, set to scrt-rng's contract addr
    let sender = deps.api.canonical_address(&env.message.sender)?;
    if sender != config.rng_addr /* & (sender !=  config.rng_interf_addr) */ {
        return Err(StdError::generic_err(
            "receive_rn did not approve sender address",
        ));
    }

    let cb_msg_deserialized = String::from_utf8(cb_msg.as_slice().to_vec()).unwrap();  // <-- will only display properly if the cb_msg input is a String
    let log_output = vec![
        log("rn", format!("{:?}",rn)),
        log("cb_msg", cb_msg_deserialized),
    ];  

    // let consumer_output = format!("Original message: {:?}, combined with rn: {:?}", 
    // String::from_utf8(cb_msg.as_slice().to_vec()),   // <-- will only display properly if the cb_msg input is a String
    // rn);

    Ok(HandleResponse {
        messages: vec![],
        log: log_output,
        data: None,
    })
    // Ok(HandleResponse::default())
}

pub fn try_trigger_create_rn<S: Storage, A: Api, Q:Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    entropy: String, 
    cb_msg: Binary, 
    receiver_code_hash: String, 
    receiver_addr: Option<String>, 
    purpose: Option<String>, 
    max_blk_delay: Option<u64>,
    rng_hash: String,
    rng_addr: String,
) -> StdResult<HandleResponse> {
    let create_rn_msg = CreateRnMsg::CreateRn { 
        entropy: entropy, 
        cb_msg: cb_msg, 
        receiver_code_hash: receiver_code_hash, 
        receiver_addr: receiver_addr, 
        purpose: purpose, 
        max_blk_delay: max_blk_delay,
    };

    let cosmos_msg = create_rn_msg.to_cosmos_msg(
        rng_hash, 
        HumanAddr(rng_addr), 
        None,
    )?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data:None,
    })
}

pub fn try_trigger_fulfill_rn<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>, 
    _env: Env, 
    creator_addr: String,
    receiver_code_hash: String,
    purpose: Option<String>,
    rng_hash: String, 
    rng_addr: String,
) -> StdResult<HandleResponse> {
    let fulfill_rn_msg = FulfillRnMsg::FulfillRn {
        creator_addr: creator_addr,
        receiver_code_hash: receiver_code_hash,
        purpose: purpose,
    };

    let cosmos_msg = fulfill_rn_msg.to_cosmos_msg(
        rng_hash,
        HumanAddr(rng_addr),
        None
    )?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data:None,
    })
}

pub fn try_receive_fulfill_rn<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>, 
    _env: Env, 
    rn: [u8; 32], 
    cb_msg: Binary, 
    purpose: Option<String>
) -> StdResult<HandleResponse> {
    debug_print!("RN user::try_receive_transmit_rn: initiated");
    let cb_msg_deserialized = String::from_utf8(cb_msg.as_slice().to_vec()).unwrap(); // from_binary::<String>(&cb_msg)?;
    debug_print!("RN user::try_receive_transmit_rn: cb_msg deserialized");
    let log_output = vec![
        log("rn", format!("{:?}",rn)),
        log("cb_msg", cb_msg_deserialized),
        log("purpose", format!("{:?}",purpose)),
    ]; 
    debug_print!("RN user::try_receive_transmit_rn: log_output created");

    Ok(HandleResponse {
        messages: vec![],
        log: log_output,
        data:None,
    })
} 

pub fn try_trigger_generate_vk<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>, 
    _env: Env, 
    // entropy: String, 
    receiver_code_hash: String, 
    // padding: Option<String>, 
    rng_hash: String, 
    rng_addr: String,
) -> StdResult<HandleResponse> {
    let entropy = "any entropy here".to_string();
    let gen_vk_msg = GenerateViewingKeyMsg::GenerateViewingKey {
        entropy,
        receiver_code_hash,
        padding: None,
    };

    let cosmos_msg = gen_vk_msg.to_cosmos_msg(
        rng_hash,
        HumanAddr(rng_addr),
        None
    )?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![],
        data: None,
    })
}

pub fn try_receive_viewing_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>, 
    _env: Env, 
    key: ViewingKey,
) -> StdResult<HandleResponse> {
    let mut keys = config_read_vk(&deps.storage).load()?;
    keys.vks.push(key.clone());

    config_vk(&mut deps.storage).save(&keys)?;
    
    let log_output = vec![
        log("added vk", format!("{:}", key)),
        log("current vks", format!("{:?}", keys.vks)),
    ]; 

    Ok(HandleResponse {
        messages: vec![],
        log: log_output,
        data: None,
    })
    // Ok(HandleResponse::default())
}

pub fn try_trigger_query_rn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>, 
    env: Env, 
    entropy: String,
    optionalvk: Option<String>, // if None, then uses vk in storage, which is the anticipated design of future scrt-rng contracts. Some() functionality for debugging only 
    // addr: HumanAddr,
    // vk: String,
) -> StdResult<HandleResponse> {
    let state = config_read(&mut deps.storage).load()?;
    let vk_store = config_read_vk(&mut deps.storage).load()?;
    let vk = match optionalvk {
        Some(i) => i,
        None => vk_store.vks[0].to_string(),
    };
    debug_print!("vk being used is: {}", &vk);

    let query_msg = QueryRnMsg::QueryRn {entropy: entropy, addr: env.contract.address, vk: vk};
    let query_ans: QueryAnswerMsg = query_msg.query(   //: StdResult<Binary>   QueryAnswerMsg 
        &deps.querier, 
        state.rng_hash, 
        deps.api.human_address(&state.rng_addr)?,
    )?;

    let output_log = format!("{:?}", query_ans.rn_output.rn);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("output", output_log)],
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

