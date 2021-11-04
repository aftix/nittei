use crate::app::AppRoute;
use crate::consts;
use crate::timers;
use gloo::storage::{self, LocalStorage, SessionStorage, Storage};
use gloo::timers::future::TimeoutFuture;
use nittei_common::auth::{
    AuthToken, PersistLoginRequest, PersistLoginResponse, PersistRequest, PersistResponse,
    PersistToken, RenewResponse,
};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

// Try to login with remember me token, or renew session token
pub fn try_login() {
    spawn_local(async {
        // First, try to renew the session
        let session: storage::Result<AuthToken> = SessionStorage::get("session_key");
        if session.is_err() {
            // No session, try to login with persistent token
            try_login_persist().await;
            return;
        }

        // There is a session, let's try to renew it
        let session = session.unwrap();
        let jwt = session.to_string();

        let req = Request::get(&format!("{}/auth/renew", consts::URL))
            .header("Content-Type", "application/x-renew-request")
            .header("Authorization", &format!("Bearer {}", jwt));
        let resp = req.send().await;
        if resp.is_err() {
            // No connection, try again later (5 seconds)
            TimeoutFuture::new(1000 * 5).await;
            try_login();
            return;
        }
        let resp = resp.unwrap();

        if resp.status() != 200 && resp.status() != 401 {
            // Failed to login! Try again soon
            TimeoutFuture::new(1000 * 5).await;
            try_login();
            return;
        } else if resp.status() == 401 {
            SessionStorage::delete("session_key");
            SessionStorage::delete("session");
            try_login();
        }

        let resp = resp.text().await;
        if resp.is_err() {
            // Failed to get text! Try again soon
            TimeoutFuture::new(1000 * 5).await;
            try_login();
            return;
        }

        let resp = ron::from_str::<RenewResponse>(&resp.unwrap());
        if resp.is_err() {
            TimeoutFuture::new(1000 * 5).await;
            try_login();
            return;
        }
        let resp = resp.unwrap();

        if let RenewResponse::Success(token) = resp {
            // Update the session key!
            SessionStorage::set("session_key", token).expect("Token not set");
            return;
        }

        // Failed to update key. We must be invalid. Remove the key, and try to login
        // again (check if there is remember me token)
        SessionStorage::delete("session_key");
        SessionStorage::delete("session");
        try_login();
    });
}

// Try to login with persistent token
pub async fn try_login_persist() {
    // Check if there is a remember me token
    let remember_me: storage::Result<PersistentLogin> = LocalStorage::get("persist");
    if remember_me.is_err() {
        return;
    }
    let remember_me = remember_me.unwrap();

    // Have a remember me token, request a login
    let ron_req = PersistLoginRequest {
        username: remember_me.user,
        token: remember_me.token,
    };
    let ron_req = ron::to_string(&ron_req);
    if ron_req.is_err() {
        return;
    }
    let req = Request::post(&format!("{}/auth/persist_login", consts::URL))
        .header("Content-Type", "application/x-persist-login-request")
        .body(&ron_req.unwrap());
    let resp = req.send().await;
    if resp.is_err() {
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

    let resp = ron::from_str::<PersistLoginResponse>(&resp.unwrap());
    if resp.is_err() {
        return;
    }
    let resp = resp.unwrap();

    if let PersistLoginResponse::Success(token, claim) = resp {
        SessionStorage::set("session", claim).expect("Failed to set session");
        SessionStorage::set("session_key", token).expect("Failed to set session key");
        timers::session_refresh();
    }

    // Oh no! Our remember me token is invalid! Delete it.
    LocalStorage::delete("persist");
}

// Check to see if we should treat the user as logged in.
pub fn logged_in() -> bool {
    let persist: storage::Result<PersistentLogin> = LocalStorage::get("persist");
    let session: storage::Result<AuthToken> = SessionStorage::get("session_key");
    persist.is_ok() || session.is_ok()
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PersistentLogin {
    user: String,
    token: PersistToken,
}

pub fn request_persistence(username: String, password: String) {
    spawn_local(async move {
        request_persistence_inner(username, password).await;
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
