use crate::app::AppRoute;
use crate::nav::{Anchor, Nav};
use crate::util::logged_in;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerifyMsg {
    Clicked,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VerifyState {
    New,
    NotLoggedIn,
    ExpiredCode,
    BadCode,
    Disconnected,
    Verified,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Properties)]
pub struct VerifyProps {
    pub code: u128,
}

pub struct Verify {
    state: VerifyState,
    code: u128,
}

impl Component for Verify {
    type Message = VerifyMsg;
    type Properties = VerifyProps;

    fn create(ctx: &Context<Self>) -> Self {
        if !logged_in() {
            Self {
                state: VerifyState::NotLoggedIn,
                code: ctx.props().code,
            }
        } else {
            Self {
                state: VerifyState::New,
                code: ctx.props().code,
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Nav route={AppRoute::Verify { code: self.code }} />
                <main id="verify" class="content">
                    {
                        match self.state {
                            VerifyState::New => {
                                let cb = ctx.link().callback(|_: MouseEvent| VerifyMsg::Clicked);
                                html_nested! {
                                    <>
                                        <button onclick={cb} id="verifybutton">{ "Verify Email" }</button>
                                    </>
                                }
                            },
                            VerifyState::NotLoggedIn => html_nested! {
                                <>
                                    <p class="failuretext">
                                        { "Not logged in!" }
                                    </p>
                                    <Anchor route={AppRoute::LoginVerify{code: self.code }}>{ "Login" }</Anchor>
                                </>
                            },
                            VerifyState::BadCode => html_nested! {
                                <>
                                    <p class="failuretext">
                                        { "Invalid verification code!" }
                                    </p>
                                </>
                            },
                            VerifyState::ExpiredCode => html_nested! {
                                <>
                                    <p class="failuretext">
                                        { "Verification code expired!" }
                                    </p>
                                </>
                            },
                            VerifyState::Disconnected => html_nested! {
                                <>
                                    <p class="failuretext">
                                        { "Can not connect to server!" }
                                    </p>
                                </>
                            },
                            VerifyState::Verified => html_nested! {
                                <>
                                    <p>
                                        { "Successfully verified!" }
                                    </p>
                                </>
                            }
                        }
                    }
                </main>
            </>
        }
    }
}
