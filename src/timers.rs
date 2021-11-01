use crate::consts;
use gloo::storage::{SessionStorage, Storage};
use gloo::timers::future::TimeoutFuture;
use nittei_common::auth::{AuthToken, RenewResponse};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

// Start a timer to refresh the session
pub fn session_refresh() {
    spawn_local(async {
        TimeoutFuture::new(1000 * 60 * 4).await;
        session_refresh_loop().await;
    });
}

// Loop to renew session keys.
async fn session_refresh_loop() {
    let key: gloo::storage::Result<AuthToken> = SessionStorage::get("session_key");
    if key.is_err() {
        return;
    }
    let key = key.unwrap();
    let jwt = key.to_string();

    let req = Request::get(&format!("{}/auth/renew", consts::URL))
        .header("Content-Type", "application/x-renew-request")
        .header("Authorization", &format!("Bearer {}", jwt));
    let resp = req.send().await;
    if resp.is_err() {
        // No connection, try again in 3 seconds
        spawn_local(async {
            TimeoutFuture::new(1000 * 3).await;
            session_refresh_loop().await;
        });
        return;
    }
    let resp = resp.unwrap();
    if resp.status() != 200 {
        return;
    }
    let resp = resp.text().await;
    if resp.is_err() {
        return;
    }
    let resp = resp.unwrap();

    let resp = ron::from_str::<RenewResponse>(&resp);
    if resp.is_err() {
        return;
    }
    let resp = resp.unwrap();

    if SessionStorage::set("session_key", resp).is_err() {
        return;
    }

    // Renew succeeded, do it again in 4 minutes
    spawn_local(async {
        TimeoutFuture::new(1000 * 60 * 4).await;
        session_refresh_loop().await;
    });
}
