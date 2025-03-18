use convert_case::Casing;
use dioxus::prelude::*;

use rand::{rng, seq::SliceRandom};
use regs_tt::{
    resources::data::{GOODS_VEHICLE, LIGHT_MOTOR_VEHICLE, PUBLIC_SERVICE},
    ui::{
        components::Icon,
        images::icon::{CAR, TAXI, TRUCK},
        views::{Home, Quiz, Results},
    },
};

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

const FAVICON: Asset = asset!("/assets/favicon.ico");
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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

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

const START_MENU_CSS: Asset = asset!("/assets/start_menu.css");

#[component]
pub fn StartMenu() -> Element {
    let mut rng = rng();
    let quizzes: Vec<regs_tt::models::Quiz> = vec![
        regs_tt::models::Quiz {
            title: "Light Motor Vehicle",
            icon: Some(CAR),
            questions: serde_json::from_str(LIGHT_MOTOR_VEHICLE).unwrap(),
        },
        regs_tt::models::Quiz {
            title: "Goods Vehicle (Heavy & Extra Heavy)",
            icon: Some(TRUCK),
            questions: serde_json::from_str(GOODS_VEHICLE).unwrap(),
        },
        regs_tt::models::Quiz {
            title: "Public Service Vehicle (Taxi)",
            icon: Some(TAXI),
            questions: serde_json::from_str(PUBLIC_SERVICE).unwrap(),
        },
    ]
    .into_iter()
    .map(|mut quiz| {
        quiz.questions.shuffle(&mut rng);
        quiz
    })
    .collect::<Vec<regs_tt::models::Quiz>>();

    rsx! {
        document::Link { rel: "stylesheet", href: START_MENU_CSS }
        section { id: "start-menu-container",
            div { id: "welcome-text",
                h1 { id: "main-title",
                    "Welcome to the "
                    span { id: "highlighted-text", "Regs TT!" }
                }
                p { id: "instruction-text", "Pick a subject to get started." }
            }

            div { id: "quiz-topic-container",
                for quiz in &quizzes {
                    QuizTopic { quiz: quiz.clone() }
                }
            }
        }
    }
}
