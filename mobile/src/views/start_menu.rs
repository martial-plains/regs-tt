use dioxus::prelude::*;
use rand::{rng, seq::SliceRandom};
use regs_tt::{
    models::Quiz,
    resources::data::{GOODS_VEHICLE, LIGHT_MOTOR_VEHICLE, PUBLIC_SERVICE},
    ui::images::icon::{CAR, TAXI, TRUCK},
};

use crate::QuizTopic;

const START_MENU_CSS: Asset = asset!("/assets/start_menu.css");

#[component]
pub fn StartMenu() -> Element {
    let mut rng = rng();
    let quizzes: Vec<Quiz> = vec![
        Quiz {
            title: "Light Motor Vehicle",
            icon: Some(CAR),
            questions: serde_json::from_str(LIGHT_MOTOR_VEHICLE).unwrap(),
        },
        Quiz {
            title: "Goods Vehicle (Heavy & Extra Heavy)",
            icon: Some(TRUCK),
            questions: serde_json::from_str(GOODS_VEHICLE).unwrap(),
        },
        Quiz {
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
    .collect::<Vec<Quiz>>();

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
