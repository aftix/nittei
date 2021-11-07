use crate::home::Home;
use crate::login::Login;
use crate::register::Register;
use crate::track::Track;
use crate::util;
use crate::verify::Verify;
use html_escape::encode_text;
use url_escape::decode;
use yew::prelude::*;
use yew_router::prelude::*;

// All the routes available to the user
#[derive(Routable, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login/verify/:code")]
    LoginVerify { code: u128 },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/account")]
    Account,
    #[at("/user/:username")]
    UserPage { username: String },
    #[at("/verify/:code")]
    Verify { code: u128 },
    #[at("/feed")]
    Feed,
    #[at("/about")]
    About,
    #[at("/donate")]
    Donate,
    #[at("/track")]
    Track,
    #[at("/404")]
    PageNotFound,
    #[at("/")]
    Home,
}

impl Into<String> for AppRoute {
    fn into(self) -> String {
        match self {
            AppRoute::LoginVerify { code: _ } => String::from("Login"),
            AppRoute::Login => String::from("Login"),
            AppRoute::Register => String::from("Register"),
            AppRoute::Account => String::from("Account"),
            AppRoute::UserPage { username: s } => encode_text(&decode(&s)).to_string(),
            AppRoute::Verify { code } => format!("verify {}", code.to_string()),
            AppRoute::Feed => String::from("Feed"),
            AppRoute::About => String::from("About"),
            AppRoute::Donate => String::from("Donate"),
            AppRoute::Track => String::from("Track"),
            AppRoute::Home => String::from("Home"),
            AppRoute::PageNotFound => String::from("Not Found"),
        }
    }
}

// QOL
pub type AppRouter = Router<AppRoute>;

// Main page messages
pub enum Msg {}

// Overall page component containing everything else
pub struct Main {}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        util::try_login();
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Render whatever page the URL bar says to
        html! {
            <AppRouter render={AppRouter::render(switch)} />
        }
    }
}

// Display the right component based on what route is in use
fn switch(switch: &AppRoute) -> Html {
    match switch {
        AppRoute::Home => html! { <Home /> },
        AppRoute::LoginVerify { code } => {
            html! { <Login href={Some(AppRoute::Verify { code: *code })} /> }
        }
        AppRoute::Login => html! { <Login /> },
        AppRoute::Register => html! { <Register /> },
        AppRoute::Verify { code } => html! { <Verify code={*code} /> },
        AppRoute::Track => html! { <Track /> },
        _ => html! { <Home /> },
    }
}
