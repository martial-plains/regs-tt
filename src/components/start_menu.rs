use dioxus::prelude::*;

use crate::{components::quiz_topic::QuizTopic, types::Quiz};

#[component]
pub fn StartMenu() -> Element {
    let quizzes: Vec<Quiz> = vec![
        Quiz {
            title: "Light Motor Vehicle",
            icon: asset!("assets/images/icon-car.svg"),
            questions: serde_json::from_str(include_str!(".././../data/light-motor-vehicle.json"))
                .unwrap(),
        },
        Quiz {
            title: "Goods Vehicle (Heavy & Extra Heavy)",
            icon: asset!("assets/images/icon-truck.svg"),
            questions: serde_json::from_str(include_str!(".././../data/goods-vehicle.json"))
                .unwrap(),
        },
        Quiz {
            title: "Public Service Vehicle (Taxi)",
            icon: asset!("assets/images/icon-taxi.svg"),
            questions: serde_json::from_str(include_str!(".././../data/public-service.json"))
                .unwrap(),
        },
    ];

    rsx! {
        section { class: "flex flex-col xl:flex-row justify-between",
            div {
                h1 { class: "text-darkNavy dark:text-white",
                    "Welcome to the "
                    span { class: "block font-medium", "Regs TT!" }
                }
                p { class: "text-greyNavy dark:text-lightBluish", "Pick a subject to get started." }
            }

            div { class: "mt-10 flex flex-col gap-3 md:mt-16 md:gap-6 xl:mt-0",
                for quiz in &quizzes {
                    QuizTopic { quiz: quiz.clone() }
                }
            }
        }
    }
}
