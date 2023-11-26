use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if (payload.username != "demo1" || payload.password != "welcome") {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create te success body.
    let body = Json(json!({
        "result": {
            "success": "true"
        }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
