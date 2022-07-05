use candid::{CandidType, Principal};
use serde::Deserialize;
use std::{collections::HashMap, vec};

#[derive(Default, CandidType, Deserialize)]
pub struct NodeStore {
    pub nodes: HashMap<Principal, PriceData>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct PriceData {
    provider: Principal,
    asset: u64,
    price: u64,
    timestamp: u64,
    is_closed: bool,
    signature: Vec<u8>,
}

impl NodeStore {}
