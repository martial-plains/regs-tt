use dioxus::prelude::*;

use crate::types::Quiz;

#[component]
pub fn Icon(quiz: ReadOnlySignal<Quiz>) -> Element {
    rsx! {
        div { class: "p-1 rounded-md md:rounded-xl md:w-14 md:h-14 md:p-2 xl:rounded-lg",
            img { src: quiz().icon, alt: quiz().icon }
        }
    }
}
