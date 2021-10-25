use yew_router::Switch;

#[derive(Switch, Clone, Copy, PartialEq, Eq)]
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
    #[to = "/"]
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
        }
    }
}
