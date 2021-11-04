use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavMsg {
    Hide,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavProps {
    #[prop_or(AppRoute::Home)]
    pub route: AppRoute,
}

pub struct Nav {
    props: NavProps,
    active: bool,
}

impl Component for Nav {
    type Message = NavMsg;
    type Properties = NavProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().to_owned(),
            active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavMsg::Hide => {
                self.active = !self.active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let home_class = if self.props.route == AppRoute::Home {
            "navcurrent"
        } else {
            ""
        };
        let track_class = if self.props.route == AppRoute::Track {
            "navcurrent"
        } else {
            ""
        };
        let feed_class = if self.props.route == AppRoute::Feed {
            "navcurrent"
        } else {
            ""
        };
        let about_class = if self.props.route == AppRoute::About {
            "navcurrent"
        } else {
            ""
        };
        let donate_class = if self.props.route == AppRoute::Donate {
            "navcurrent"
        } else {
            ""
        };
        let login_class = if self.props.route == AppRoute::Login {
            "navcurrent"
        } else {
            ""
        };

        let nav_class = if self.active { "navactive" } else { "" };
        let home_text: String = AppRoute::Home.into();
        let track_text: String = AppRoute::Track.into();
        let feed_text: String = AppRoute::Feed.into();
        let about_text: String = AppRoute::About.into();
        let donate_text: String = AppRoute::Donate.into();
        let login_text: String = AppRoute::Login.into();

        html! {
            <nav class={nav_class}>
                <button onclick={ctx.link().callback(|_| NavMsg::Hide)}>
                    <img src="res/minilogo.svg" alt="Nittei Mini Logo" />
                </button>
                <ul>
                    <li class={home_class}><Anchor route={AppRoute::Home}>{ home_text }</Anchor></li>
                    <li class={track_class}><Anchor route={AppRoute::Track}>{ track_text }</Anchor></li>
                    <li class={feed_class}><Anchor route={AppRoute::Feed}>{ feed_text }</Anchor></li>
                    <li class={about_class}><Anchor route={AppRoute::About}>{ about_text }</Anchor></li>
                    <li class={donate_class}><Anchor route={AppRoute::Donate}>{ donate_text }</Anchor></li>
                    <li class={login_class}><Anchor route={AppRoute::Login}>{ login_text }</Anchor></li>
                </ul>
            </nav>
        }
    }
}

pub type Anchor = Link<AppRoute>;
