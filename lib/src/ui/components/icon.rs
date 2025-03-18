use dioxus::prelude::*;

use crate::models::Quiz;

const ICON_CSS: Asset = asset!("/assets/styling/components/icon.css");

#[component]
pub fn Icon(
    quiz: ReadOnlySignal<Quiz>,
    #[props(default = String::new(), into)] class: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ICON_CSS }

        div { id: "quiz-icon-wrapper", class: "{class}",
            if let Some(icon) = quiz().icon {
                img { src: icon, alt: quiz().title }
            }
        }
    }
}
