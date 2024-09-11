use dioxus::prelude::*;

pub mod home;
pub mod quiz;
pub mod results;

use home::Home;
use quiz::Quiz;
use results::Results;
use start_menu::StartMenu;

use crate::components::start_menu;

#[derive(Clone, Routable, Debug, PartialEq, Eq)]
pub enum Route {
    #[layout(Home)]
    #[route("/")]
    StartMenu {},
    #[route("/quiz/:title")]
    Quiz { title: String },
    #[route("/quiz/:title/result")]
    Results { title: String },
}
