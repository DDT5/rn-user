use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

use crate::viewing_key::ViewingKey;

pub static CONFIG_KEY: &[u8] = b"config";
pub const VK_STORE_KEY: &[u8] = b"viewing key"; // store the viewing key from scrt-rng to enable querying

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub rng_hash: String,
    pub rng_addr: CanonicalAddr,
    // pub rng_interf_addr: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct VkStore {
    pub vks: Vec<ViewingKey>,
}

pub fn config_vk<S: Storage>(storage: &mut S) -> Singleton<S, VkStore> {
    singleton(storage, VK_STORE_KEY)
}

pub fn config_read_vk<S: Storage>(storage: &S) -> ReadonlySingleton<S, VkStore> {
    singleton_read(storage, VK_STORE_KEY)
}
