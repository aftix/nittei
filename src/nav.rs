use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavMsg {}

#[derive(Clone, PartialEq, Properties)]
pub struct NavProps {
    #[prop_or(AppRoute::Home)]
    pub route: AppRoute,
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
        let home_class = if self.props.route == AppRoute::Home {
            "navcurrent"
        } else {
            "navnotcurrent"
        };
        let track_class = if self.props.route == AppRoute::Track {
            "navcurrent"
        } else {
            "navnotcurrent"
        };
        let feed_class = if self.props.route == AppRoute::Feed {
            "navcurrent"
        } else {
            "navnotcurrent"
        };
        let about_class = if self.props.route == AppRoute::About {
            "navcurrent"
        } else {
            "navnotcurrent"
        };
        let donate_class = if self.props.route == AppRoute::Donate {
            "navcurrent"
        } else {
            "navnotcurrent"
        };
        let login_class = if self.props.route == AppRoute::Login {
            "navcurrent"
        } else {
            "navnotcurrent"
        };

        let home_text: String = AppRoute::Home.into();
        let track_text: String = AppRoute::Track.into();
        let feed_text: String = AppRoute::Feed.into();
        let about_text: String = AppRoute::About.into();
        let donate_text: String = AppRoute::Donate.into();
        let login_text: String = AppRoute::Login.into();

        html! {
            <nav>
                <ul>
                    <li class=home_class><Anchor route=AppRoute::Home>{ home_text }</Anchor></li>
                    <li class=track_class><Anchor route=AppRoute::Track>{ track_text }</Anchor></li>
                    <li class=feed_class><Anchor route=AppRoute::Feed>{ feed_text }</Anchor></li>
                    <li class=about_class><Anchor route=AppRoute::About>{ about_text }</Anchor></li>
                    <li class=donate_class><Anchor route=AppRoute::Donate>{ donate_text }</Anchor></li>
                    <li class=login_class><Anchor route=AppRoute::Login>{ login_text }</Anchor></li>
                </ul>
            </nav>
        }
    }
}

pub type Anchor = RouterAnchor<AppRoute>;
