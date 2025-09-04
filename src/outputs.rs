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
    pub bids: Vec<[u32; 2]>,
    pub asks: Vec<[u32; 2]>,
    pub lastUpdatedId: String
}