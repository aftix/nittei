pub mod consts;
pub mod timers;

pub mod util;

// The overall app, all the pages, etc
pub mod app;

// The navigation bar
pub mod nav;

// Home page
pub mod home;

// Tracking page
pub mod track;

// Login page
pub mod login;

// Registration page
pub mod register;

fn main() {
    yew::start_app::<app::Main>();
}
