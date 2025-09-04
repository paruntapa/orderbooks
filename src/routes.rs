use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, get, http::StatusCode, post, web::{Data, Json}};
use crate::{inputs::{CreateOrderInput, Delete_Order}, orderbook::OrderBook, outputs::{CreationResponse, DeleteOrderResponse, Depth}};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<OrderBook>>>) -> impl Responder {

    let _price = body.0.price;
    let _quantity = body.0.quantity;
    let _user_id = body.0.user_id;
    let _side = body.0.side;
    
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.create_order(_price, _quantity, _user_id, _side);

    let r = CreationResponse {
        order_id: String::from("Order placed")
    };

    return (HttpResponse::Ok().json(r), StatusCode::OK);
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<Delete_Order>, orderbook: Data<OrderBook>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<OrderBook>) -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        lastUpdatedId: String::from("8768asd78as78d9a7s9da98ds9a87a98sd")
    })
}