use crate::app::AppRoute;
use crate::consts;
use crate::nav::{Anchor, Nav};
use crate::timers;
use gloo::storage::{SessionStorage, Storage};
use nittei_common::auth::*;
use reqwasm::http::Request;
use ron;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yewtil::future::LinkFuture;

#[derive(Clone, Debug)]
pub enum LoginMsg {
    Login,
    LoginRecieved(LoginResponse),
    LoginFailed,
    Disconnected,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LoginState {
    Normal,
    Disconnected,
    Failed,
    Missing,
    LockedOut,
    BadUser,
    BadPassword,
}

impl Into<String> for LoginState {
    fn into(self) -> String {
        match self {
            LoginState::Normal => String::new(),
            LoginState::Disconnected => String::from("Disconnected from network"),
            LoginState::Failed => String::from("Internal Server Error"),
            LoginState::Missing => String::from("Missing field value!"),
            LoginState::LockedOut => String::from("Too many attempts. Please wait."),
            LoginState::BadUser => String::from("Invalid username!"),
            LoginState::BadPassword => String::from("Invalid password!"),
        }
    }
}

pub struct Login {
    link: ComponentLink<Self>,
    state: LoginState,
    userref: NodeRef,
    passref: NodeRef,
}

// use reqwasm to do a login API call
async fn login_request(username: String, password: String) -> LoginMsg {
    let ron_request = LoginRequest { username, password };
    let ron_request = ron::to_string(&ron_request).expect("Should serialize");
    let req = Request::post(&format!("{}/auth/login", consts::URL))
        .header("Content-Type", "application/x-login-request")
        .body(&ron_request);
    let resp = req.send().await;
    if resp.is_err() {
        return LoginMsg::Disconnected;
    }
    let resp = resp.unwrap();
    if resp.status() != 200 {
        return LoginMsg::LoginFailed;
    }

    let resp = resp.text().await;
    if resp.is_err() {
        return LoginMsg::LoginFailed;
    }
    let resp = resp.unwrap();

    let resp = ron::from_str::<LoginResponse>(&resp);
    if resp.is_err() {
        LoginMsg::LoginFailed
    } else {
        LoginMsg::LoginRecieved(resp.unwrap())
    }
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: LoginState::Normal,
            userref: NodeRef::default(),
            passref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginMsg::Login => {
                let userref = self.userref.clone();
                let passref = self.passref.clone();
                self.link.send_future(async move {
                    let userbox = userref.cast::<HtmlInputElement>();
                    let passbox = passref.cast::<HtmlInputElement>();
                    if userbox.is_none() || passbox.is_none() {
                        return LoginMsg::LoginFailed;
                    }
                    let userbox = userbox.unwrap();
                    let passbox = passbox.unwrap();
                    let username = userbox.value();
                    let password = passbox.value();
                    if username.len() == 0 || password.len() == 0 {
                        return LoginMsg::Missing;
                    }
                    login_request(username, password).await
                });
                false
            }
            LoginMsg::Disconnected => {
                self.state = LoginState::Disconnected;
                true
            }
            LoginMsg::LoginRecieved(resp) => {
                match resp {
                    LoginResponse::InvalidRequest => self.state = LoginState::Failed,
                    LoginResponse::LockedOut => self.state = LoginState::LockedOut,
                    LoginResponse::PasswordWrong => self.state = LoginState::BadPassword,
                    LoginResponse::UsernameInvalid => self.state = LoginState::BadUser,
                    LoginResponse::Success(token) => {
                        if SessionStorage::set("session_key", token).is_err() {
                            self.state = LoginState::Failed;
                        } else {
                            timers::session_refresh();
                            let window = web_sys::window().expect("No window exists");
                            window.location().set_href("/").expect("Failed to navigate");
                        }
                    }
                };
                true
            }
            LoginMsg::LoginFailed => {
                self.state = LoginState::Failed;
                true
            }
            LoginMsg::Missing => {
                self.state = LoginState::Missing;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // When the login button is clicked, don't submit the form, but message this component
        let cb = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            LoginMsg::Login
        });
        let failure_text: String = self.state.into(); // Error to display
        html! {
            <>
                <Nav route=AppRoute::Login />
                <main id="login" class="content">
                    <form id="loginform">
                        <label for="unamebox">{ "Username" }</label>
                        <input type="text" id="unamebox" name="username" ref=self.userref.clone() />
                        <label for="passbox">{ "Password" }</label>
                        <input type="password" id="passbox" name="password" ref=self.passref.clone() />
                        <button id="loginsubmit" type="submit" onclick=cb>{ "Log In" }</button>
                    </form>
                    // Only display error if text is not empty
                    <p class="failuretext" style=if failure_text.len() == 0 { "display: none;" } else { "" }>
                        { failure_text }
                    </p>
                    <p>
                        {"Don't have an account?"}
                        <Anchor route=AppRoute::Register>{ "Sign Up" }</Anchor>
                    </p>
                </main>
            </>
        }
    }
}
