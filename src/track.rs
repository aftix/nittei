use crate::app::AppRoute;
use crate::nav::Nav;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrackMsg {}

pub struct Track {}

impl Component for Track {
    type Message = TrackMsg;
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
                <Nav route={AppRoute::Track} />
                <main class="content" id="track">
                </main>
            </>
        }
    }
}
