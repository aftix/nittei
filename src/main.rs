// The overall app, all the pages, etc
pub mod app;

// The navigation bar
pub mod nav;

// Home page
pub mod home;

// Tracking page
pub mod track;

fn main() {
    yew::start_app::<app::Main>();
}
