use crate::home::Home;
use crate::login::Login;
use crate::register::Register;
use crate::track::Track;
use html_escape::encode_text;
use url_escape::decode;
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

// All the routes available to the user
#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/account"]
    Account,
    #[to = "/user/{username}"]
    UserPage(String),
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
            AppRoute::UserPage(s) => encode_text(&decode(&s)).to_string(),
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

// QOL
pub type AppRouter = Router<AppRoute>;

// Main page messages
pub enum Msg {}

// Overall page component containing everything else
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
        // Render whatever page the URL bar says to
        html! {
            <AppRouter render=AppRouter::render(Self::switch) redirect=AppRouter::redirect(|route: Route| {
                AppRoute::PageNotFound(Permissive(Some(route.route)))
            }) />
        }
    }
}

impl Main {
    // Display the right component based on what route is in use
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
