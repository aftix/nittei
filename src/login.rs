use crate::app::AppRoute;
use crate::nav::{Anchor, Nav};
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LoginMsg {
    Login,
}

pub struct Login {
    link: ComponentLink<Self>,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <>
                <Nav route=AppRoute::Login />
                <main id="login" class="content">
                    <form id="loginform">
                        <label for="unamebox">{ "Username" }</label>
                        <input type="text" id="unamebox" name="username" />
                        <label for="passbox">{ "Password" }</label>
                        <input type="password" id="passbox" name="password" />
                        <button id="loginsubmit" type="submit" onclick=cb>{ "Log In" }</button>
                    </form>
                    <p>
                        {"Don't have an account?"}
                        <Anchor route=AppRoute::Register>{ "Sign Up" }</Anchor>
                    </p>
                </main>
            </>
        }
    }
}
