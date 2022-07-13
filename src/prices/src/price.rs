use candid::{CandidType, Principal};
use serde::Deserialize;
use std::{collections::HashMap, vec};

#[derive(Default, CandidType, Deserialize)]
pub struct Node2Price {
    pub map: HashMap<Principal, PriceData>,
}

impl Node2Price {}

#[derive(CandidType, Deserialize, Clone)]
pub struct PriceData {
    provider: Principal,
    asset: u32,
    price: u32,
    timestamp: u32,
    is_closed: bool,
    signature: String,
}
