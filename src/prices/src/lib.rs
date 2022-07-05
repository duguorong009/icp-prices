use crate::state::STATE;
use candid::Principal;
use ic_cdk_macros::*;

use price::PriceData;

mod price;
mod state;

#[init]
fn main() {
    let caller = ic_cdk::caller();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.owner = Some(caller);
    });
}

#[update]
fn add_node(node: Principal) -> Option<Principal> {
    let caller = ic_cdk::caller();
    return STATE.with(|s| s.borrow_mut().add_node(node, caller));
}

#[update]
fn remove_node(node: Principal) -> Option<Principal> {
    let caller = ic_cdk::caller();
    return STATE.with(|s| s.borrow_mut().remove_node(node, caller));
}

#[update]
fn add_data(asset: u64, data: PriceData) -> bool {
    let caller = ic_cdk::caller();
    return STATE.with(|s| s.borrow_mut().add_data(asset, data, caller));
}

#[query]
fn get_data(asset: u64) -> Vec<PriceData> {
    return STATE.with(|s| {
        let state = s.borrow();
        return state.get_data(asset);
    });
}
