use dioxus::prelude::*;

use crate::{
    components::{button::Button, icon::Icon},
    types::Quiz,
};

#[component]
pub fn Results(title: String) -> Element {
    let mut score = use_context::<Signal<i32>>();
    let mut quiz = use_context::<Signal<Quiz>>();

    let mut handle_play_again = move || {
        score.set(0);
        quiz.set(Quiz::default());
        navigator().replace("/");
    };

    rsx! {
        section { class: "flex flex-col xl:flex-row xl:gap-8 xl:justify-between",
            div {
                p { class: "text-darkNavy dark:text-white text-heading-l-mobile font-light md:text-heading-l-regular",
                    "Quiz completed "
                    span { class: "black font-medium", "You scored..." }
                }
            }

            div { class: "xl:min-w-[520px]",
                div { class: "bg-white dark:bg-navy drop-shadow-light dark:drop-shadow-dark mt-10 mb-3 p-8 h-fit w-full rounded-xl flex flex-col items-center justify-center gap-4 md:mt-16 md:mb-8 md:p-12 md:rounded-3xl md:gap-10 xl:mt-0",

                    div { class: "flex items-center gap-4 md:gap-6",
                        Icon { quiz }

                        p { class: "text-darkNavy dark:text-white text-heading-s-mobile font-medium md:text-heading-s",

                            {title}
                        }
                    }

                    p { class: "text-darkNavy dark:text-white text-display-mobile font-medium md:text-display",

                        {score().to_string()}
                    }

                    p { class: "text-greyNavy dark:text-lightBluish text-body-m-mobile font-regular md:text-body-m",

                        {format!("out of {}", quiz().questions.len())}
                    }
                }

                Button { title: "Play Again", onclick: move |_| handle_play_again() }
            }
        }
    }
}
