use crate::home::Home;
use crate::login::Login;
use crate::register::Register;
use crate::track::Track;
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/account"]
    Account,
    #[to = "/feed"]
    Feed,
    #[to = "/about"]
    About,
    #[to = "/donate"]
    Donate,
    #[to = "/track"]
    Track,
    #[to = "/404"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

impl Into<String> for AppRoute {
    fn into(self) -> String {
        match self {
            AppRoute::Login => String::from("Login"),
            AppRoute::Register => String::from("Register"),
            AppRoute::Account => String::from("Account"),
            AppRoute::Feed => String::from("Feed"),
            AppRoute::About => String::from("About"),
            AppRoute::Donate => String::from("Donate"),
            AppRoute::Track => String::from("Track"),
            AppRoute::Home => String::from("Home"),
            AppRoute::PageNotFound(perm) => {
                String::from(format!("PageNotFound {}", perm.0.unwrap()))
            }
        }
    }
}

pub type AppRouter = Router<AppRoute>;

pub enum Msg {}

pub struct Main {
    _link: ComponentLink<Self>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Main { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter render=AppRouter::render(Self::switch) redirect=AppRouter::redirect(|route: Route| {
                AppRoute::PageNotFound(Permissive(Some(route.route)))
            }) />
        }
    }
}

impl Main {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Home => html! { <Home /> },
            AppRoute::Login => html! { <Login /> },
            AppRoute::Register => html! { <Register /> },
            AppRoute::Track => html! { <Track /> },
            _ => html! { <Home /> },
        }
    }
}
