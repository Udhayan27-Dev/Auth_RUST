use serde::{Deserialize,Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct User{
    pub uid: String,
    pub email:String,
    pub pw:String,
    pub role: String,
}       

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email:String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub token:String,
}

#[tokio::main]
async fn main(){
    let user = Arc::new(intit_users());
}
