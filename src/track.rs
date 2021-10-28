use crate::app::AppRoute;
use crate::nav::Nav;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrackMsg {}

pub struct Track {
    _link: ComponentLink<Self>,
}

impl Component for Track {
    type Message = TrackMsg;
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
            <div id="track" class="content">
                <Nav route=AppRoute::Track />
            </div>
        }
    }
}
