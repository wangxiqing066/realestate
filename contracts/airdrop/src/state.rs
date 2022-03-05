use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub terraland_token: Addr,
    pub fee_config: Vec<FeeConfig>,
    pub mission_smart_contracts: MissionSmartContracts,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MissionSmartContracts {
    pub lp_staking: Option<Addr>,
    pub tland_staking: Option<Addr>,
    pub platform_registry: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FeeConfig {
    pub fee: Uint128,
    pub operation: String,
    pub denom: String,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Member {
    pub amount: Uint128,
    pub claimed: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct State {
    pub num_of_members: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MEMBERS: Map<&Addr, Member> = Map::new("members");
pub const STATE: Item<State> = Item::new("state");
