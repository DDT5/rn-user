use cosmwasm_std::{Binary, HumanAddr, }; //CanonicalAddr
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::utils::{HandleCallback, Query};

use crate::viewing_key::ViewingKey; //pad_handle_result, pad_query_result, Query,  

const BLOCK_SIZE: usize = 256;


/////////////////////////////////////////////////////////////////////////////////
// Init
/////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub rng_hash: String,
    pub rng_addr: String,
    // pub rng_interf_addr: String,
}

/////////////////////////////////////////////////////////////////////////////////
// Handles
/////////////////////////////////////////////////////////////////////////////////

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
    TriggerGenerateVk {/*entropy: String,*/ receiver_code_hash: String, /*padding: Option<String>,*/ rng_hash: String, rng_addr: String,},
    ReceiveViewingKey {key: ViewingKey,},
    TriggerQueryRn {entropy: String, optionalvk: Option<String>},
}

// ------------------------------------------------------------------------------
// Enums for callback
// ------------------------------------------------------------------------------

// Calling handle in another contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackRnMsg {
    CallbackRn {entropy: String, cb_msg: Binary, callback_code_hash: String, contract_addr: String},
}

impl HandleCallback for CallbackRnMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CreateRnMsg {
    CreateRn {
        entropy: String, cb_msg: Binary, receiver_code_hash: String, 
        receiver_addr: Option<String>, purpose: Option<String>, max_blk_delay: Option<u64>,
    },
}

impl HandleCallback for CreateRnMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FulfillRnMsg {
    FulfillRn {creator_addr: String, receiver_code_hash: String, purpose: Option<String>},
}

impl HandleCallback for FulfillRnMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GenerateViewingKeyMsg {
    GenerateViewingKey {entropy: String, receiver_code_hash: String, padding: Option<String>,},
}

impl HandleCallback for GenerateViewingKeyMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

// Calling query in another contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryRnMsg {
    QueryRn {entropy: String, addr: HumanAddr, vk: String},
}

impl Query for QueryRnMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryAnswerMsg {
    pub rn_output: RnOutput,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct RnOutput {
    pub rn: [u8; 32],
}


/////////////////////////////////////////////////////////////////////////////////
// Queries
/////////////////////////////////////////////////////////////////////////////////

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
