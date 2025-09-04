use actix_web::{HttpResponse, Responder, delete, get, http::StatusCode, post, web::Json};
use crate::{inputs::{CreateOrderInput, Delete_Order}, orderbook::orderbook, outputs::{CreationResponse, DeleteOrderResponse, Depth}};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {

    let _price = body.0.price;
    let _quantity = body.0.quantity;
    let _user_id = body.0.user_id;
    let _side = body.0.side;


    let r = CreationResponse {
        order_id: String::from("Order placed")
    };

    return (HttpResponse::Ok().json(r), StatusCode::OK);
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<Delete_Order>) -> impl Responder {
    let order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse {
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        lastUpdatedId: String::from("8768asd78as78d9a7s9da98ds9a87a98sd")
    })
}