use cosmwasm_std::{Binary, HumanAddr, }; //CanonicalAddr
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub rng_addr: String,
    pub rng_interf_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CallRn {entropy:String, cb_msg:Binary, rng_hash: String, rng_addr: String},
    ReceiveRn {rn: [u8; 32], cb_msg: Binary},
    ReceiveFRn {rn: [u8; 32], cb_msg: Binary, purpose: Option<String>},
    TriggerCreateRn {
        entropy: String, cb_msg: Binary, receiver_code_hash: String, 
        receiver_addr: Option<String>, purpose: Option<String>, max_blk_delay: Option<u64>,
        rng_hash: String, rng_addr: String,
    },
    TriggerFulfillRn {creator_addr: String, receiver_code_hash: String, purpose: Option<String>, rng_hash: String, rng_addr: String,},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetRngAddr {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryResponse {
    pub rng_addr: HumanAddr,
}
