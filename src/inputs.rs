use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug)]

pub struct CreateOrderInput {
    pub price: String,
    pub quantity: u32,
    pub user_id: Number,
    pub side: Side
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Side {
    Bids,
    Asks
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Delete_Order {
    pub order_id: String
}