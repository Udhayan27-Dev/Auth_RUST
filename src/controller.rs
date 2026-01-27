use axum::Json;
use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use crate::model::{LoginInfo, LoginResponse,Claims};
use axum::http::StatusCode;

pub async fn login_handler(Json(login_info) : Json<LoginInfo>) -> Result<Json<LoginResponse>,StatusCode>{
    let username = &login_info.username;
    let password = &login_info.password;

    let valid_user = is_valid_user(username, password);
    if (valid_user){
        let claims = Claims{
            sub:username.clone(),
            exp:(chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
        };
        let token = match encode(&Header::default(),&claims,&EncodingKey::from_secret("secretkey".as_ref()))
            {
                Ok(tok)=> tok,
                Err(e)=>{
                    eprintln!("Error Generating the Token {}",e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
            };
        Ok(Json(LoginResponse{token}))
    }
    else{
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub fn is_valid_user(username:&str,password:&str)  ->bool{
    //need to implement DB to verify the USER .....(mocked code)
    username != "" && password != ""
}

pub async fn get_info_handler(header_map:HeaderMap) -> Result<Json<String>,StatusCode>{
    if let Some(auth_header) = header_map.get("Authorization"){
        if let Ok(auth_header_str) =auth_header.to_str(){
            if auth_header_str.starts_with("Bearer "){
                let token = auth_header_str.trim_start_matchs("Bearer "){
                    return match decode::<Claims>(&token,&DecodingKey::from_secret("secret".as_ref()), &Validation::default()){
                        Ok(_) => {
                            let info = "You are valid here in Info".to_string();
                            return Ok(Json(info));
                        }
                        Err(e) => {
                            eprintln!("Error Generatind Token {}",e);
                            return Err(StatusCode::UNAUTHORIZED)
                        }
                    }
                }
            }
        }
    }
}
