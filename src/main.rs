use crate::controller::{get_info_handler, login_handler};
use axum::{Router,routing::{get,post}};

mod model;
mod controller;

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/info",get(get_info_handler));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is Listening....");
    
    axum::serve(listener,app).await.unwrap();
    
    
            
}