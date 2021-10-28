use crate::app::AppRoute;
use crate::nav::{Anchor, Nav};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LoginMsg {}

pub struct Login {
    _link: ComponentLink<Self>,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Nav route=AppRoute::Login />
                <main id="login" class="content">
                    <form>
                        <label for="unamebox">{"Username"}</label>
                        <input type="text" id="unamebox" name="username"/>
                        <label for="passbox">{"Password"}</label>
                        <input type="password" id="passbox" name="password"/>
                        <input type="submit" value="Log In" id="loginsubmit" />
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
