use dioxus::prelude::*;

const BUTTON_CSS: Asset = asset!("/assets/styling/components/button.css");

#[component]
pub fn Button(
    #[props(into)] id: Option<&'static str>,
    #[props(into)] onclick: EventHandler<MouseEvent>,
    #[props(into)] title: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BUTTON_CSS }

        button { id, onclick, {title} }
    }
}
