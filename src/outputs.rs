use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreationResponse {
    pub order_id: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteOrderResponse {
    pub filled_qty: u32,
    pub average_price: u32
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Depth {
    pub price: f64,
    pub quantity: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepthResponse {
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}