use crate::consts;
use gloo::storage::{self, LocalStorage, SessionStorage, Storage};
use nittei_common::auth::{AuthToken, PersistRequest, PersistResponse, PersistToken};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys;

#[derive(Serialize, Debug, Deserialize)]
pub struct PersistentLogin {
    user: String,
    token: PersistToken,
}

pub fn request_persistence(username: String, password: String, href: &str) {
    let href = String::from(href);
    spawn_local(async move {
        request_persistence_inner(username, password).await;
        if href != "" {
            let window = web_sys::window().expect("No window");
            window
                .location()
                .set_href(&href)
                .expect("Failed to navigate");
        }
    });
}

async fn request_persistence_inner(username: String, password: String) {
    let jwt: storage::Result<AuthToken> = SessionStorage::get("session_key");
    if jwt.is_err() {
        console_web::error!("Persist: No session found.");
        return;
    }
    let jwt = jwt.unwrap().to_string();

    let req = PersistRequest {
        username: username.clone(),
        password,
    };
    let req = ron::to_string(&req);
    if req.is_err() {
        console_web::error!("Persist: RON Serialization error.");
        return;
    }
    let req = req.unwrap();

    let req = Request::post(&format!("{}/auth/persist_request", consts::URL))
        .header("Content-Type", "application/x-persist-request")
        .header("Authorization", &format!("Bearer {}", jwt))
        .body(req);
    let resp = req.send().await;
    if resp.is_err() {
        console_web::error!("Persist: Request error.");
        return;
    }
    let resp = resp.unwrap();

    if resp.status() != 200 {
        console_web::error!("Persist: HTTP Not Ok");
        return;
    }

    let resp = resp.text().await;
    if resp.is_err() {
        console_web::error!("Persist: Text not Ok");
        return;
    }
    let resp = resp.unwrap();

    let resp = ron::from_str::<PersistResponse>(&resp);
    if resp.is_err() {
        console_web::error!("Persist: Deserialization error.");
        return;
    }
    if let PersistResponse::Success(token) = resp.unwrap() {
        let persist = PersistentLogin {
            user: username,
            token,
        };

        LocalStorage::set("persist", persist).expect("Failed to save persist token");
    } else {
        console_web::error!("Persist: Bad Response");
    }
}
