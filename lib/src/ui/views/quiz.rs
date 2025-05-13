use std::sync::Arc;

use crate::ui::{
    components::Button,
    images::icon::{CORRECT, INCORRECT},
};
use convert_case::Casing;
use dioxus::prelude::*;

const QUIZ_CSS: Asset = asset!("/assets/styling/views/quiz.css");

#[component]
pub fn Quiz(title: String) -> Element {
    let mut number = use_signal(|| 0);
    let mut answer = use_signal(String::new);
    let mut is_submitted = use_signal(|| false);
    let mut feedback = use_signal(|| "");
    let mut score = use_context::<Signal<i32>>();
    let mut show_error = use_signal(|| false);
    let quiz = use_context::<Signal<crate::models::Quiz>>();

    let handle_answer = move |selected: String| {
        if feedback().is_empty() {
            answer.set(selected);
        }
    };

    let mut handle_submit = {
        move || {
            if answer().is_empty() {
                show_error.set(true);
                return;
            }

            if is_submitted() {
                if quiz().questions.len() == number() + 1 {
                    navigator().replace(format!(
                        "/quiz/{}/result",
                        title.to_case(convert_case::Case::Kebab)
                    ));
                }

                number.set(number() + 1);
                answer.set(String::new());
                is_submitted.set(false);
                feedback.set("");
                show_error.set(false);
            } else {
                let is_correct = answer() == quiz().questions[number()].answer;
                feedback.set(if is_correct { "Correct!" } else { "Wrong!" });
                score.set(if is_correct { score() + 1 } else { score() });
                is_submitted.set(true);
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: QUIZ_CSS }
        section { id: "quiz-section",
            div { id: "progress-container",
                progress {
                    id: "progress-bar",

                    value: i32::try_from(number + 1).unwrap(),
                    max: i32::try_from(quiz().questions.len()).unwrap(),
                }

                p { id: "question-info",

                    {format!("Question {} of {}", number + 1, quiz().questions.len())}
                }

                p { id: "question-stem", {quiz().questions[number()].stem.clone()} }
            }

            ul { id: "options-list",
                {
                    quiz()
                        .questions[number()]
                        .options
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(index, option)| {
                            rsx! {
                                QuizOption {
                                    key: index,
                                    option: option.clone(),
                                    index,
                                    on_submit: handle_answer,
                                    selected_answer: answer,
                                    feedback,
                                    true_answer: quiz().questions[number()].answer.clone(),
                                }
                            }
                        })
                }
            }

            Button {
                id: "next-question-button",
                onclick: move |_| handle_submit(),
                title: if !is_submitted() { "Submit Answer" } else { if quiz().questions.len() == number + 1 { "Finish Quiz" } else { "Next Question" }
                    .to_string() },
            }

            if show_error() {
                div { id: "error-message",
                    img { src: INCORRECT, alt: "incorrect" }
                    p { id: "error-text", "Please select an answer" }
                }
            }
        }
    }
}

const QUIZ_OPTION_CSS: Asset = asset!("/assets/styling/views/quiz_option.css");

#[component]
fn QuizOption(
    option: String,
    index: usize,
    on_submit: EventHandler<String>,
    selected_answer: String,
    feedback: String,
    true_answer: String,
) -> Element {
    let selected_answer = Arc::new(selected_answer);
    let option = Arc::new(option);
    let true_answer = Arc::new(true_answer);

    let get_option_label = |index| {
        let letters = ['A', 'B', 'C', 'D'];
        letters[index]
    };

    let option_label = get_option_label(index);

    let get_border_color = {
        let seletect_answer = selected_answer.clone();
        let feedback = feedback.clone();
        let option = option.clone();
        move || {
            if seletect_answer.is_empty() || feedback.is_empty() {
                return if seletect_answer == option {
                    "correct"
                } else {
                    ""
                };
            }

            if seletect_answer == option {
                return if feedback == "Correct!" {
                    "correct"
                } else {
                    "incorrect"
                };
            }

            ""
        }
    };

    let get_background_color = {
        let selected_answer = selected_answer.clone();
        let option = option.clone();
        let feedback = feedback.clone();

        move || {
            if selected_answer == option && feedback.is_empty() {
                return "bg-purple";
            }

            if selected_answer == option {
                return if feedback == "Correct!" {
                    "bg-green"
                } else {
                    "bg-red"
                };
            }

            "bg-light-grey"
        }
    };

    let status = {
        let selected_answer = selected_answer.clone();
        let option = option.clone();
        let feedback = feedback.clone();

        move || {
            if !feedback.is_empty() && true_answer == option {
                return rsx! {
                    img { src: CORRECT, alt: "correct", class: "correct-icon" }
                };
            }

            if feedback == "Wrong!" && selected_answer == option {
                return rsx! {
                    img {
                        src: INCORRECT,
                        alt: "incorrect",
                        class: "incorrect-icon",
                    }
                };
            }

            rsx!()
        }
    };

    let option_id = format!("quiz-option-{index}"); // Dynamically generate the ID based on the index.

    rsx! {
        document::Link { rel: "stylesheet", href: QUIZ_OPTION_CSS }
        li {
            id: option_id, // Use the dynamically generated ID
            class: format!("quiz-option {}", get_border_color()),
            onclick: {
                let option = option.clone();
                move |_| on_submit(option.to_string())
            },
            div {
                class: format!(
                    "{} text-body {}",
                    get_background_color(),
                    if selected_answer == option { "selected" } else { "" },
                ),
                p { class: if selected_answer == option { "text-white" } else { "" },
                    {option_label.to_string()}
                }
            }
            p { class: "feedback", {<std::string::String as Clone>::clone(&option)} }
            {status()}
        }
    }
}
