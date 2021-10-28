use crate::app::AppRoute;
use crate::nav::Nav;
use yew::prelude::*;

mod action;
mod what;
mod why;
use action::*;
use what::*;
use why::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HomeMsg {}

pub struct Home {
    _link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = HomeMsg;
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
                <Nav route=AppRoute::Home />
                <main id="home" class="content">
                    <What />
                    <Why />
                    <Action />
                </main>
            </>
        }
    }
}
