use std::sync::Arc;

use convert_case::Casing;
use dioxus::prelude::*;

use crate::components::button::Button;

#[component]
pub fn Quiz(title: String) -> Element {
    let mut number = use_signal(|| 0);
    let mut answer = use_signal(String::new);
    let mut is_submitted = use_signal(|| false);
    let mut feedback = use_signal(|| "");
    let mut score = use_context::<Signal<i32>>();
    let mut show_error = use_signal(|| false);
    let quiz = use_context::<Signal<crate::types::Quiz>>();

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
        section { class: "flex flex-col xl:justify-between xl:gap-8",
            div { class: "xl:relative",
                progress {
                    class: "text-white dark:text-inherit w-full progress-bar xl:left-0",

                    value: i32::try_from(number() + 1).unwrap(),
                    max: i32::try_from(quiz().questions.len()).unwrap()
                }

                p { class: "text-greyNavy dark:text-lightBluish text-body-s-mobile italic md:text-body-s",

                    {format!("Question {} of {}", number + 1, quiz().questions.len())}
                }

                p { class: "text-darkNavy dark:text-white mt-3 mb-6 font-medium text-heading-m-mobile md:text-heading-m md:mt-7 md:mb-10",

                    { quiz().questions[number()].stem.clone() }
                }
            }

            ul { class: "mt-10 flex flex-col gap-3 mb-3 md:mt-16 md:mb-8 md:gap-6 xl:mt-0",
                {quiz().questions[number()].options.iter().enumerate().map(|(index, option)| {
                    rsx! {
                        Option {
                            key: index,
                            option: option.clone(),
                            index: index,
                            on_submit: handle_answer,
                            selected_answer: answer,
                            feedback: feedback,
                            true_answer: quiz().questions[number()].answer.clone()
                        }
                    }
                })}
            }

            Button {
                onclick: move |_| handle_submit(),
                title: if is_submitted() {
                    "Submit Answer"
                } else {
                    if quiz().questions.len() == number + 1 {
                        "Finish Quiz"
                    } else {
                        "Next Question"
                    }
                }
            }

            if show_error() {
                div { class: "mb-2 flex items-center justify-center gap-2",
                    img {
                        src: asset!("/assets/images/icon-incorrect.svg"),
                        alt: "incorrect"
                    }
                    p { class: "text-redAccent dark:text-white text-body-m-mobile md:text-body-m",
                        "Please select an answer"
                    }
                }
            }
        }
    }
}

#[component]
fn Option(
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
                    "border-purpleAccent border-[3px]"
                } else {
                    ""
                };
            }

            if seletect_answer == option {
                return if feedback == "Correct!" {
                    "border-greenAccent border-[3px]"
                } else {
                    "border-redAccent border-[3px]"
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
                return "bg-purpleAccent ";
            }

            if selected_answer == option {
                return if feedback == "Correct!" {
                    "bg-greenAccent"
                } else {
                    "bg-redAccent"
                };
            }

            "bg-lightGrey"
        }
    };

    let status = {
        let selected_answer = selected_answer.clone();
        let option = option.clone();
        let feedback = feedback.clone();

        move || {
            if !feedback.is_empty() && true_answer == option {
                return rsx! {
                    img {
                        src: asset!("/assets/images/icon-correct.svg"),
                        alt: "correct",
                        class: "absolute right-3"
                    }
                };
            }

            if feedback == "Wrong!" && selected_answer == option {
                return rsx! {
                    img {
                        src: asset!("/assets/images/icon-incorrect.svg"),
                        alt: "incorrect",
                        class: "absolute right-3"
                    }
                };
            }

            rsx!()
        }
    };

    rsx! {
        li {
            class: format!(
                "bg-white dark:bg-navy drop-shadow-light dark:drop-shadow-dark {} h-16 p-3 flex items-center rounded-xl cursor-pointer gap-4 relative group md:h-20 md:p-3 md:gap-8 md:rounded-3xl xl:h-[92px] xl:min-w-[520px] h-fit",
                get_border_color(),
            ),
            onclick: {
                let option = option.clone();
                move |_| on_submit(option.to_string())
            },
            div {
                class: format!(
                    "{} {} text-body-m-mobile font-medium px-[14px] py-3 rounded-md  md:w-14 md:h-14 md:px-[18px] md:py-[14px] md:rounded-xl xl:rounded-lg",
                    get_background_color(),
                    if selected_answer == option { "" } else { "group-hover:bg-[#F6E7FF]" },
                ),
                p {
                    class: format!(
                        "{} {} text-greyNavy text-body-m-mobile font-medium md:text-heading-s",
                        if selected_answer == option { "text-white" } else { "" },
                        if selected_answer == option { "" } else { "group-hover:text-purpleAccent" },
                    ),
                    {option_label.to_string()}
                }
            }
            p {
                class: format!(
                    "text-darkNavy dark:text-white {}",
                    if feedback.is_empty() { "" } else { "pr-9" },
                ),
                {<std::string::String as Clone>::clone(&option)}
            }
            {status()}
        }
    }
}
