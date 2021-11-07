use crate::app::AppRoute;
use crate::consts;
use crate::nav::{Anchor, Nav};
use crate::timers;
use crate::util;
use gloo::storage::{SessionStorage, Storage};
use nittei_common::auth::*;
use reqwasm::http::Request;
use ron;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug)]
pub enum LoginMsg {
    Login,
    LoginRecieved(LoginResponse),
    LoginFailed,
    Disconnected,
    Missing,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoginProps {
    pub href: Option<AppRoute>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LoginState {
    Normal,
    Disconnected,
    Failed,
    Missing,
    LockedOut,
    BadEmail,
    BadPassword,
}

impl From<LoginState> for String {
    fn from(state: LoginState) -> String {
        match state {
            LoginState::Normal => String::new(),
            LoginState::Disconnected => String::from("Disconnected from network"),
            LoginState::Failed => String::from("Internal Server Error"),
            LoginState::Missing => String::from("Missing field value!"),
            LoginState::LockedOut => String::from("Too many attempts. Please wait."),
            LoginState::BadEmail => String::from("Invalid email address!"),
            LoginState::BadPassword => String::from("Invalid password!"),
        }
    }
}

pub struct Login {
    state: LoginState,
    href: Option<AppRoute>,
    emailref: NodeRef,
    passref: NodeRef,
    rememberref: NodeRef,
}

// use reqwasm to do a login API call
async fn login_request(email: String, password: String) -> LoginMsg {
    let ron_request = LoginRequest { email, password };
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
    type Properties = LoginProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: LoginState::Normal,
            href: ctx.props().href.clone(),
            emailref: NodeRef::default(),
            passref: NodeRef::default(),
            rememberref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::Login => {
                let emailref = self.emailref.clone();
                let passref = self.passref.clone();
                ctx.link().send_future(async move {
                    let userbox = emailref.cast::<HtmlInputElement>();
                    let passbox = passref.cast::<HtmlInputElement>();
                    if userbox.is_none() || passbox.is_none() {
                        return LoginMsg::LoginFailed;
                    }
                    let userbox = userbox.unwrap();
                    let passbox = passbox.unwrap();
                    let username = userbox.value();
                    let password = passbox.value();
                    if username.is_empty() || password.is_empty() {
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
                    LoginResponse::EmailInvalid => self.state = LoginState::BadEmail,
                    LoginResponse::Success(token, claim) => {
                        if SessionStorage::set("session_key", token).is_err()
                            || SessionStorage::set("session", claim).is_err()
                        {
                            self.state = LoginState::Failed;
                        } else {
                            timers::session_refresh();
                            let checkbox = self.rememberref.cast::<HtmlInputElement>();
                            let user = self.emailref.cast::<HtmlInputElement>();
                            let pass = self.passref.cast::<HtmlInputElement>();

                            if let Some(user) = user {
                                if let Some(pass) = pass {
                                    if let Some(check) = checkbox {
                                        if check.checked() {
                                            util::request_persistence(user.value(), pass.value());
                                        }
                                    }
                                }
                            }

                            yew_router::replace_route(self.href.clone().unwrap_or(AppRoute::Home));
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        // When the login button is clicked, don't submit the form, but message this component
        let cb = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            LoginMsg::Login
        });
        let failure_text: String = self.state.into(); // Error to display
        html! {
            <>
                <Nav route={AppRoute::Login} />
                <main id="login" class="content">
                    <form id="loginform">
                        <label for="emailbox">{ "Email" }</label>
                        <input type="text" id="emailbox" name="email" ref={self.emailref.clone()} />
                        <label for="passbox">{ "Password" }</label>
                        <input type="password" id="passbox" name="password" ref={self.passref.clone()} />
                        <div id="login-remember">
                            <input type="checkbox" id="rememberme" name="rememberme" ref={self.rememberref.clone()} />
                            <label for="rememberme">{ "Remember Me" }</label>
                        </div>
                        <button id="loginsubmit" type="submit" onclick={cb}>{ "Log In" }</button>
                    </form>
                    // Only display error if text is not empty
                    <p class="failuretext" style={if failure_text.is_empty() { "display: none;" } else { "" }}>
                        { failure_text }
                    </p>
                    <p>
                        {"Don't have an account?"}
                        <Anchor route={AppRoute::Register}>{ "Sign Up" }</Anchor>
                    </p>
                </main>
            </>
        }
    }
}
