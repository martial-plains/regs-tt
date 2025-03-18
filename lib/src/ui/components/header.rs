use dioxus::prelude::*;

use crate::{models::Quiz, ui::components::Icon};

const HEADER_CSS: Asset = asset!("/assets/styling/components/header.css");

#[component]
pub fn Header() -> Element {
    let quiz = use_context::<Signal<Quiz>>();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER_CSS }

        header { id: "header",
            div { id: "header-content",
                Icon { quiz }
                p { id: "quiz-title", {quiz().title} }
            }
        }
    }
}
