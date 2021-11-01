use crate::consts;
use gloo::storage::{SessionStorage, Storage};
use gloo::timers::future::TimeoutFuture;
use nittei_common::auth::{AuthToken, RenewResponse};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

// Start a timer to refresh the session
pub fn session_refresh() {
    // immediately refresh since time isn't implemented on WASM we can't tell if it's needed
    spawn_local(async move {
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
        // No connection, try again in 10 seconds
        spawn_local(async {
            TimeoutFuture::new(1000 * 10).await;
            session_refresh_loop().await;
        });
        return;
    }
    let resp = resp.unwrap();
    // Since we use the presence of a session as proof of login, delete it so we won't get confused
    if resp.status() == 401 {
        SessionStorage::delete("session-key");
        SessionStorage::delete("session");
    }
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

    if let RenewResponse::Success(jwt) = resp {
        if SessionStorage::set("session_key", jwt).is_err() {
            return;
        }
    } else {
        return;
    }

    // Renew succeeded, do it again in 4 minutes
    spawn_local(async {
        TimeoutFuture::new(1000 * 60 * 4).await;
        session_refresh_loop().await;
    });
}
