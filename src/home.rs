use crate::app::AppRoute;
use crate::nav::Nav;
use yew::prelude::*;

// Components for use on home page
mod action;
mod what;
mod why;
use action::*;
use what::*;
use why::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HomeMsg {}

pub struct Home {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Nav route={AppRoute::Home} />
                <main id="home" class="content">
                    <What />
                    <Why />
                    <Action />
                </main>
            </>
        }
    }
}
