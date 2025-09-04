use std::collections::HashMap;

use crate::{inputs::Side, outputs::Depth};

pub struct OrderBook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
    pub order_id_index: u32
}

pub struct UserOrder {
    pub user_id: u32,
    pub qty: u32,
    pub order_id: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index: 0
        }
    }
}

impl OrderBook {
    pub fn create_order(&mut self, _price: u32, _quantity: u32, _user_id: u32, _Side: Side) {
        let order_id = self.order_id_index;
        self.order_id_index = self.order_id_index + 1;

        if _Side == Side::Buy {
            self.bids.entry(_price).or_insert(Vec::new()).push(UserOrder { 
                user_id: _user_id,
                qty: _quantity, 
                order_id
            });
        } else {
            self.asks.entry(_price).or_insert(Vec::new()).push(UserOrder {
                user_id: _user_id,
                qty: _quantity, 
                order_id
            })
        }
    }

    pub fn get_depth(&self, _user_id: u32) -> Depth {
        let mut bids_vec = Vec::new();
        let mut asks_vec = Vec::new();

        // Convert bids HashMap to vector of [price, quantity] arrays
        for (price, orders) in &self.bids {
            let total_qty: u32 = orders.iter().map(|order| order.qty).sum();
            bids_vec.push([*price, total_qty]);
        }

        // Convert asks HashMap to vector of [price, quantity] arrays
        for (price, orders) in &self.asks {
            let total_qty: u32 = orders.iter().map(|order| order.qty).sum();
            asks_vec.push([*price, total_qty]);
        }

        // Sort bids in descending order (highest price first)
        bids_vec.sort_by(|a, b| b[0].cmp(&a[0]));
        
        // Sort asks in ascending order (lowest price first)
        asks_vec.sort_by(|a, b| a[0].cmp(&b[0]));

        Depth {
            bids: bids_vec,
            asks: asks_vec,
            lastUpdatedId: format!("orderbook_{}", _user_id)
        }
    }
}