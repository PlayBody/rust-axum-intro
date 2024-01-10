
use axum::{Json, Router, routing::post};
use serde_json::{Value, json};
use serde::Deserialize;
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<12} - api_login", "HANDLER");
  // Todo: Implement real db/auth logic.
  if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
  }
  // Todo: Implement real auth-token
  cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

  // Create the success body.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));
  Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}