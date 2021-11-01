use crate::app::AppRoute;
use crate::nav::Nav;
use email_address_parser::EmailAddress;
use passwords::{analyzer, scorer};
use web_sys::{HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RegisterMsg {
    Register,
    EmailTyped,
    UserTyped,
    PasswordTyped,
    Password2Typed,
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
}

pub struct Register {
    link: ComponentLink<Self>,
    emailref: NodeRef,
    userref: NodeRef,
    passref: NodeRef,
    pass2ref: NodeRef,
    state: RegisterState,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            emailref: NodeRef::default(),
            userref: NodeRef::default(),
            passref: NodeRef::default(),
            pass2ref: NodeRef::default(),
            state: RegisterState::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                if email.len() == 0 || !EmailAddress::is_valid(&email, None) {
                    self.state.bademail = true;
                    fail = true;
                }

                let user = userbox.unwrap().value();
                if user.len() == 0 || user.len() > 30 {
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

                if scored < 80.0 {
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

                // TODO: Send register request

                false
            }
            RegisterMsg::EmailTyped => {
                let emailbox = self.emailref.cast::<HtmlInputElement>();
                if emailbox.is_none() {
                    return false;
                }
                let email = emailbox.unwrap().value();
                if email.len() == 0 || !EmailAddress::is_valid(&email, None) {
                    let laststate = self.state.bademail;
                    self.state.bademail = true;
                    if laststate != true {
                        true
                    } else {
                        false
                    }
                } else {
                    let laststate = self.state.bademail;
                    self.state.bademail = false;
                    if laststate != false {
                        true
                    } else {
                        false
                    }
                }
            }
            RegisterMsg::UserTyped => {
                let userbox = self.userref.cast::<HtmlInputElement>();
                if userbox.is_none() {
                    return false;
                }
                let userbox = userbox.unwrap();
                let username = userbox.value();
                if username.len() == 0 || username.len() > 30 {
                    let laststate = self.state.baduser;
                    self.state.baduser = true;
                    if laststate != true {
                        true
                    } else {
                        false
                    }
                } else {
                    let laststate = self.state.baduser;
                    self.state.baduser = false;
                    if laststate != false {
                        true
                    } else {
                        false
                    }
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
                    if laststate != true {
                        mismatched_render = true;
                    }
                } else {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = false;
                    if laststate != false {
                        mismatched_render = true;
                    }
                }

                if password.len() < 8 {
                    let laststate = self.state.shortpassword;
                    self.state.shortpassword = true;
                    self.state.longpassword = false;
                    if laststate != true {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    let laststate = self.state.shortpassword;
                    self.state.shortpassword = false;
                    if laststate != false {
                        mismatched_render = true;
                    }
                }

                if password.len() > 30 {
                    let laststate = self.state.longpassword;
                    self.state.longpassword = true;
                    if laststate != true {
                        return true;
                    } else {
                        return mismatched_render;
                    }
                } else {
                    let laststate = self.state.longpassword;
                    self.state.longpassword = false;
                    if laststate != false {
                        mismatched_render = true;
                    }
                }

                let analyzed = analyzer::analyze(&password);
                let scored = scorer::score(&analyzed);

                if scored < 70.0 {
                    let laststate = self.state.badpassword;
                    self.state.badpassword = true;
                    if laststate != true {
                        true
                    } else {
                        mismatched_render
                    }
                } else {
                    let laststate = self.state.badpassword;
                    self.state.badpassword = false;
                    if laststate != false {
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
                    if laststate != true {
                        true
                    } else {
                        false
                    }
                } else {
                    let laststate = self.state.mismatched_password;
                    self.state.mismatched_password = false;
                    if laststate != false {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(|e: MouseEvent| {
            e.prevent_default();
            RegisterMsg::Register
        });

        let email_cb = self
            .link
            .callback(|_: KeyboardEvent| RegisterMsg::EmailTyped);
        let user_cb = self
            .link
            .callback(|_: KeyboardEvent| RegisterMsg::UserTyped);
        let pass_cb = self
            .link
            .callback(|_: KeyboardEvent| RegisterMsg::PasswordTyped);
        let pass2_cb = self
            .link
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
                <Nav route=AppRoute::Register />
                <main id="register" class="content">
                    <form id="registerform">
                        <label for="emailbox">{ "Email" }</label>
                        <input type="text" id="emailbox" name="email" ref=self.emailref.clone() onkeyup=email_cb />
                        <p class="failuretext" style=if self.state.bademail { "" } else { "display: none;"}>{ "Invaild email address!" }</p>
                        <label for="unamebox">{ "Username" }</label>
                        <input type="text" id="unamebox" name="username" ref=self.userref.clone() onkeyup=user_cb />
                        <p class="failuretext" style=if self.state.baduser { "" } else { "display: none;" }>{ "Invalid username!" }</p>
                        <label for="passbox">{ "Password" }</label>
                        <input type="password" id="passbox" name="password" ref=self.passref.clone() onkeyup=pass_cb />
                        <p class="failuretext" style=if self.state.shortpassword { "" } else { "display: none;" }>{ "Password must be at least 8 characters." }</p>
                        <p class="failuretext" style=if self.state.longpassword { "" } else { "display: none;" }>{ "Password must be no longer than 30 characters." }</p>
                        <p class="failuretext" style=if self.state.badpassword { "" } else { "display: none;" }>{ "Password too weak!" }</p>
                        <label for="passbox2">{ "Re-enter password" }</label>
                        <input type="password" id="passbox2" name="password2" ref=self.pass2ref.clone() onkeyup=pass2_cb />
                        <p class="failuretext" style=if self.state.mismatched_password { "" } else { "display: none;" }>{ "Mismatched passwords!" }</p>
                        <button id="registersubmit" type="submit" onclick=cb>{ "Register" }</button>
                    </form>
                    <p class="failuretext" style=if failure_text.len() == 0 { "display: none;" } else { "" }>{ failure_text }</p>
                </main>
            </>
        }
    }
}
