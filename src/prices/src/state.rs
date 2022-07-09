use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::price::{NodePriceDataMap, PriceData};

#[derive(Default, CandidType, Deserialize)]
pub struct State {
    asset_data: HashMap<u32, NodePriceDataMap>,

    pub owner: Option<Principal>,
    pub nodes: Vec<Principal>,
    pub nodes_index: HashMap<Principal, u32>,
}

impl State {
    pub fn add_node(&mut self, node: Principal, caller: Principal) -> Option<Principal> {
        if let Some(owner) = self.owner {
            if owner != caller {
                return None;
            }
        }
        let id = self.nodes_index.len();
        self.nodes_index.insert(node.clone(), id as u32);
        self.nodes.push(node);
        Some(node)
    }

    pub fn remove_node(&mut self, node: Principal, caller: Principal) -> Option<Principal> {
        if let Some(owner) = self.owner {
            if owner != caller {
                return None;
            }
        }
        let nodes_len = self.nodes.len();

        let node_id = match self.nodes_index.get(&node) {
            Some(id) => *id as usize,
            None => {
                return None;
            }
        };
        let last_node = self.nodes[nodes_len - 1];
        self.nodes_index.insert(last_node, node_id as u32);

        self.nodes[node_id] = self.nodes[nodes_len - 1];

        self.nodes_index.remove_entry(&node);

        self.nodes.pop()
    }

    pub fn add_data(&mut self, asset: u32, data: PriceData, caller: Principal) -> bool {
        // Validations
        let id = match self.nodes_index.get(&caller) {
            Some(id) => *id as usize,
            None => return false,
        };
        let node = self.nodes[id];
        if node != caller {
            return false; // should return err
        }

        // Insert data
        self.asset_data.entry(asset).and_modify(|node_price| {
            node_price.map.insert(caller, data);
        });
        true
    }

    pub fn get_data(&self, asset: u32) -> Vec<PriceData> {
        let mut res: Vec<PriceData> = vec![];
        for i in 0..self.nodes.len() {
            let node_price = self.asset_data.get(&asset).unwrap();
            let data = node_price.map.get(&self.nodes[i]).unwrap();
            res.push(data.clone());
        }
        res
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
