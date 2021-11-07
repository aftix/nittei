use crate::app::AppRoute;
use crate::consts;
use crate::nav::Nav;
use crate::util;
use email_address_parser::EmailAddress;
use gloo::storage::{SessionStorage, Storage};
use nittei_common::auth::{RegisterRequest, RegisterResponse};
use passwords::{analyzer, scorer};
use reqwasm::http::Request;
use web_sys::{HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone)]
pub enum RegisterMsg {
    Register,
    EmailTyped,
    UserTyped,
    PasswordTyped,
    Password2Typed,
    Disconnected,
    Failed,
    RegisterRecieved(RegisterResponse),
}

#[derive(Default)]
struct RegisterState {
    bademail: bool,
    baduser: bool,
    shortpassword: bool,
    longpassword: bool,
    badpassword: bool,
    mismatched_password: bool,
    disconnected: bool,
    server_error: bool,
    user_taken: bool,
    email_taken: bool,
}

pub struct Register {
    emailref: NodeRef,
    userref: NodeRef,
    passref: NodeRef,
    pass2ref: NodeRef,
    rememberref: NodeRef,
    state: RegisterState,
}

async fn register_request(email: String, username: String, password: String) -> RegisterMsg {
    let ron_request = RegisterRequest {
        email,
        username,
        password,
    };
    let ron_request = ron::to_string(&ron_request).expect("Should serialize");
    let req = Request::post(&format!("{}/auth/register", consts::URL))
        .header("Content-Type", "application/x-register-request")
        .body(&ron_request);
    let resp = req.send().await;
    if resp.is_err() {
        return RegisterMsg::Disconnected;
    }
    let resp = resp.unwrap();
    if resp.status() != 200 {
        return RegisterMsg::Failed;
    }

    let resp = resp.text().await;
    if resp.is_err() {
        return RegisterMsg::Failed;
    }

    let resp = ron::from_str::<RegisterResponse>(&resp.unwrap());
    if resp.is_err() {
        RegisterMsg::Failed
    } else {
        RegisterMsg::RegisterRecieved(resp.unwrap())
    }
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            emailref: NodeRef::default(),
            userref: NodeRef::default(),
            passref: NodeRef::default(),
            pass2ref: NodeRef::default(),
            rememberref: NodeRef::default(),
            state: RegisterState::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::Register => {
                if self.state.bademail
                    || self.state.baduser
                    || self.state.badpassword
                    || self.state.mismatched_password
                    || self.state.shortpassword
                {
                    return false;
                }

                let emailbox = self.emailref.cast::<HtmlInputElement>();
                let userbox = self.userref.cast::<HtmlInputElement>();
                let passbox = self.passref.cast::<HtmlInputElement>();
                let pass2box = self.pass2ref.cast::<HtmlInputElement>();
                if emailbox.is_none()
                    || userbox.is_none()
                    || passbox.is_none()
                    || pass2box.is_none()
                {
                    return false;
                }

                let mut fail = false;

                let email = emailbox.unwrap().value();
                if email.is_empty() || !EmailAddress::is_valid(&email, None) {
                    self.state.bademail = true;
                    fail = true;
                }

                let user = userbox.unwrap().value();
                if user.is_empty() || user.len() > 30 {
                    self.state.baduser = true;
                    fail = true;
                }

                let password = passbox.unwrap().value();
                if password.len() < 8 {
                    self.state.shortpassword = true;
                    fail = true;
                }

                let analyzed = analyzer::analyze(&password);
                let scored = scorer::score(&analyzed);

                if scored < 70.0 {
                    self.state.badpassword = true;
                    fail = true;
                }

                let password2 = pass2box.unwrap().value();
                if password2 != password {
                    self.state.mismatched_password = true;
                    fail = true;
                }

                if fail {
                    return fail;
                }

                ctx.link()
                    .send_future(async move { register_request(email, user, password).await });

                false
            }
            RegisterMsg::EmailTyped => {
                let emailbox = self.emailref.cast::<HtmlInputElement>();
                if emailbox.is_none() {
                    return false;
                }
                let email = emailbox.unwrap().value();
                if email.is_empty() || !EmailAddress::is_valid(&email, None) {
                    let laststate = self.state.bademail;
                    self.state.bademail = true;
                    !laststate
                } else {
                    let laststate = self.state.bademail;
                    self.state.bademail = false;
                    laststate
                }
            }
            RegisterMsg::UserTyped => {
                let userbox = self.userref.cast::<HtmlInputElement>();
                if userbox.is_none() {
                    return false;
                }
                let userbox = userbox.unwrap();
                let username = userbox.value();
                if username.is_empty() || username.len() > 30 {
                    let laststate = self.state.baduser;
                    self.state.baduser = true;
                    !laststate
                } else {
                    let laststate = self.state.baduser;
                    self.state.baduser = false;
                    laststate
                }
            }
            RegisterMsg::PasswordTyped => {
                let passbox = self.passref.cast::<HtmlInputElement>();
                let pass2box = self.pass2ref.cast::<HtmlInputElement>();
                if passbox.is_none() || pass2box.is_none() {
                    return false;
                }
                let password = passbox.unwrap().value();
                let password2 = pass2box.unwrap().value();

                let mut mismatched_render = false;
                if password != password2 {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = true;
                    if !laststate {
                        mismatched_render = true;
                    }
                } else {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = false;
                    if laststate {
                        mismatched_render = true;
                    }
                }

                if password.len() < 8 {
                    let laststate = self.state.shortpassword;
                    self.state.shortpassword = true;
                    self.state.longpassword = false;
                    return !laststate;
                } else {
                    let laststate = self.state.shortpassword;
                    self.state.shortpassword = false;
                    if laststate {
                        mismatched_render = true;
                    }
                }

                if password.len() > 30 {
                    let laststate = self.state.longpassword;
                    self.state.longpassword = true;
                    if !laststate {
                        return true;
                    } else {
                        return mismatched_render;
                    }
                } else {
                    let laststate = self.state.longpassword;
                    self.state.longpassword = false;
                    if laststate {
                        mismatched_render = true;
                    }
                }

                let analyzed = analyzer::analyze(&password);
                let scored = scorer::score(&analyzed);

                if scored < 70.0 {
                    let laststate = self.state.badpassword;
                    self.state.badpassword = true;
                    if !laststate {
                        true
                    } else {
                        mismatched_render
                    }
                } else {
                    let laststate = self.state.badpassword;
                    self.state.badpassword = false;
                    if laststate {
                        true
                    } else {
                        mismatched_render
                    }
                }
            }
            RegisterMsg::Password2Typed => {
                let passbox = self.passref.cast::<HtmlInputElement>();
                let pass2box = self.pass2ref.cast::<HtmlInputElement>();
                if passbox.is_none() || pass2box.is_none() {
                    return false;
                }
                let password = passbox.unwrap().value();
                let password2 = pass2box.unwrap().value();

                if password != password2 {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = true;
                    !laststate
                } else {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = false;
                    laststate
                }
            }
            RegisterMsg::Failed => {
                self.state.server_error = true;
                true
            }
            RegisterMsg::Disconnected => {
                self.state.disconnected = true;
                true
            }
            RegisterMsg::RegisterRecieved(resp) => {
                self.state.server_error = false;
                self.state.disconnected = false;
                self.state.email_taken = false;
                self.state.user_taken = false;
                self.state.baduser = false;
                self.state.badpassword = false;
                self.state.bademail = false;

                match resp {
                    RegisterResponse::UsernameTaken => {
                        self.state.user_taken = true;
                        true
                    }
                    RegisterResponse::EmailTaken => {
                        self.state.email_taken = true;
                        true
                    }
                    RegisterResponse::Lockout => {
                        self.state.server_error = true;
                        true
                    }
                    RegisterResponse::InvalidRequest => {
                        self.state.server_error = true;
                        true
                    }
                    RegisterResponse::InvalidUsername => {
                        self.state.baduser = true;
                        true
                    }
                    RegisterResponse::WeakPassword => {
                        self.state.badpassword = true;
                        true
                    }
                    RegisterResponse::InvalidEmail => {
                        self.state.bademail = true;
                        true
                    }
                    RegisterResponse::Success(token, claim) => {
                        if SessionStorage::set("session_key", token).is_err()
                            || SessionStorage::set("session", claim).is_err()
                        {
                            self.state.server_error = true;
                        } else {
                            let checkbox = self.rememberref.cast::<HtmlInputElement>();
                            let user = self.userref.cast::<HtmlInputElement>();
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
                            yew_router::replace_route(AppRoute::Home);
                        }
                        false
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            RegisterMsg::Register
        });

        let email_cb = ctx
            .link()
            .callback(|_: KeyboardEvent| RegisterMsg::EmailTyped);
        let user_cb = ctx
            .link()
            .callback(|_: KeyboardEvent| RegisterMsg::UserTyped);
        let pass_cb = ctx
            .link()
            .callback(|_: KeyboardEvent| RegisterMsg::PasswordTyped);
        let pass2_cb = ctx
            .link()
            .callback(|_: KeyboardEvent| RegisterMsg::Password2Typed);

        let failure_text = if self.state.disconnected {
            String::from("Disconnected from network")
        } else if self.state.server_error {
            String::from("Internal Server Error")
        } else {
            String::new()
        };
        html! {
            <>
                <Nav route={AppRoute::Register} />
                <main id="register" class="content">
                    <form id="registerform">
                        <label for="emailbox">{ "Email" }</label>
                        <input type="text" id="emailbox" name="email" ref={self.emailref.clone()} onkeyup={email_cb} />
                        <p class="failuretext" style={if self.state.bademail { "" } else { "display: none;"}}>{ "Invaild email address!" }</p>
                        <p class="failuretext" style={if self.state.email_taken { "" } else { "display: none;" }}>{ "Email taken!" }</p>
                        <label for="unamebox">{ "Username" }</label>
                        <input type="text" id="unamebox" name="username" ref={self.userref.clone()} onkeyup={user_cb} />
                        <p class="failuretext" style={if self.state.baduser { "" } else { "display: none;" }}>{ "Invalid username!" }</p>
                        <p class="failuretext" style={if self.state.user_taken { "" } else { "display: none;" }}>{ "Username taken!" }</p>
                        <label for="passbox">{ "Password" }</label>
                        <input type="password" id="passbox" name="password" ref={self.passref.clone()} onkeyup={pass_cb} />
                        <p class="failuretext" style={if self.state.shortpassword { "" } else { "display: none;" }}>{ "Password must be at least 8 characters." }</p>
                        <p class="failuretext" style={if self.state.longpassword { "" } else { "display: none;" }}>{ "Password must be no longer than 30 characters." }</p>
                        <p class="failuretext" style={if self.state.badpassword { "" } else { "display: none;" }}>{ "Password too weak!" }</p>
                        <label for="passbox2">{ "Re-enter password" }</label>
                        <input type="password" id="passbox2" name="password2" ref={self.pass2ref.clone()} onkeyup={pass2_cb} />
                        <p class="failuretext" style={if self.state.mismatched_password { "" } else { "display: none;" }}>{ "Mismatched passwords!" }</p>
                        <div id="register-remember">
                            <input type="checkbox" id="rememberme-register" name="rememberme" ref={self.rememberref.clone()} />
                            <label for="rememberme-register">{ "Remember Me" }</label>
                        </div>
                        <button id="registersubmit" type="submit" onclick={cb}>{ "Register" }</button>
                    </form>
                    <p class="failuretext" style={if failure_text.is_empty() { "display: none;" } else { "" }}>{ failure_text }</p>
                </main>
            </>
        }
    }
}
