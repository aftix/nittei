use crate::nav::Nav;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HomeMsg {}

pub struct Home {
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = HomeMsg;
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
        html! {
            <div id="home" class="content">
                <Nav />
            </div>
        }
    }
}
