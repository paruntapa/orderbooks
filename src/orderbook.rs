use std::collections::HashMap;

use crate::{inputs::Side, outputs::{Depth, DepthResponse}};

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
    pub fn get_depth(&self) -> DepthResponse {
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        for (price, orders) in self.bids.iter() {
            bids.push(Depth { price: *price as f64, quantity: orders.iter().map(|o| o.qty).sum::<u32>() as f64 });
        }
        for (price, orders) in self.asks.iter() {
            asks.push(Depth { price: *price as f64, quantity: orders.iter().map(|o| o.qty).sum::<u32>() as f64 });  
        }
        DepthResponse { bids, asks }
    }
}



// https://petal-estimate-4e9.notion.site/Week-3-CEXs-and-CLOBs-22f7dfd10735801aae3fda9fd3d9aa67