#![allow(clippy::derive_partial_eq_without_eq)]

use convert_case::Casing;
use dioxus::prelude::*;

use crate::{components::icon::Icon, routes::Route, types::Quiz};

#[component]
pub fn QuizTopic(quiz: Quiz) -> Element {
    let mut state_quiz = use_context::<Signal<Quiz>>();
    let quiz = Signal::new(quiz);

    rsx! {
        Link {
            to: Route::Quiz {
                title: quiz().title.to_case(convert_case::Case::Kebab),
            },
            class: "bg-white drop-shadow-light dark:bg-navy bg:drop-shadow-dark w-full h-16  rounded-xl flex items-center gap-4 pl-3 md:gap-8 md:h-20 md:rounded-3xl xl:px-5 xl:h-24 xl:min-w-[520px]",
            onclick: move |_| state_quiz.set(quiz()),
            Icon { quiz: quiz() }
            p { class: "text-darkNavy dark:text-white font-medium text-body-m-mobile md:text-heading-s",
                {quiz().title}
            }
        }
    }
}
