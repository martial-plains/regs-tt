use convert_case::Casing;
use dioxus::prelude::*;

use crate::views::StartMenu;

use regs_tt::ui::{
    components::{Icon, Navbar},
    views::{Home, Quiz, Results},
};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Home::<Route>)]
    #[route("/")]
    StartMenu {},
    #[route("/quiz/:title")]
    Quiz { title: String },
    #[route("/quiz/:title/result")]
    Results { title: String },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    use_context_provider(|| Signal::new(regs_tt::models::Quiz::default()));
    use_context_provider(|| Signal::new(0_i32));

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, user-scalable=no",
        }


        Router::<Route> {}
    }
}

const QUIZ_TOPIC_CSS: Asset = asset!("/assets/quiz_topic.css");

#[component]
pub fn QuizTopic(quiz: ReadOnlySignal<regs_tt::models::Quiz>) -> Element {
    let mut state_quiz = use_context::<Signal<regs_tt::models::Quiz>>();

    rsx! {
        document::Link { rel: "stylesheet", href: QUIZ_TOPIC_CSS }
        Link {
            to: Route::Quiz {
                title: quiz().title.to_case(convert_case::Case::Kebab),
            },
            id: "quiz-topic-link",
            onclick: move |_| state_quiz.set(quiz()),
            Icon { quiz }
            p { id: "quiz-title", {quiz().title} }
        }
    }
}

/// A mobile-specific Router around the shared `Navbar` component
/// which allows us to use the mobile-specific `Route` enum.
#[component]
fn MobileNavbar() -> Element {
    rsx! {
        Navbar {
        }

        Outlet::<Route> {}
    }
}
