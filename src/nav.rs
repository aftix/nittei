use crate::app::AppRoute;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavMsg {}

#[derive(Clone, Copy, PartialEq, Eq, Properties)]
pub struct NavProps {
    #[prop_or(AppRoute::Home)]
    route: AppRoute,
}

pub struct Nav {
    link: ComponentLink<Self>,
    props: NavProps,
}

impl Component for Nav {
    type Message = NavMsg;
    type Properties = NavProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <nav>
                <ul>
                    <li class="navcurrent">{ "Home" }</li>
                    <li>{ "Track" }</li>
                    <li>{ "Feed" }</li>
                    <li>{ "About" }</li>
                    <li>{ "Donate" }</li>
                    <li>{ "Login" }</li>
                </ul>
            </nav>
        }
    }
}
