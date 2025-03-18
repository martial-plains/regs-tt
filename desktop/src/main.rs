use convert_case::Casing;
use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};

use regs_tt::ui::{
    components::{Icon, Navbar},
    views::{Home, Quiz, Results},
};
mod views;

use crate::views::StartMenu;

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
    let cfg = dioxus::desktop::Config::new().with_window(
        WindowBuilder::new()
            .with_title("Regs TT")
            .with_always_on_top(false)
            .with_min_inner_size(LogicalSize::new(400, 600)),
    );

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    use_context_provider(|| Signal::new(regs_tt::models::Quiz::default()));
    use_context_provider(|| Signal::new(0_i32));

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "container", Router::<Route> {} }
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

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
        }

        Outlet::<Route> {}
    }
}
