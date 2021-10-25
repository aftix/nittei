// The overall app, all the pages, etc
pub mod app;

// The navigation bar
pub mod nav;

// Home page
pub mod home;

fn main() {
    yew::start_app::<home::Home>();
}
