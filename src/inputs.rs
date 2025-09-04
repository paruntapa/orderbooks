use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]

pub struct CreateOrderInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Delete_Order {
    pub order_id: String
}