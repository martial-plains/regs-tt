use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(into)] onclick: EventHandler<MouseEvent>,
    #[props(into)] title: String,
) -> Element {
    rsx! {
        button {
            class: "drop-shadow-light dark:drop-shadow-none bg-purpleAccent mb-3 h-14 w-full rounded-xl text-heading-s-mobile font-medium text-white  md:h-[92px] md:text-heading-s md:rounded-3xl md:mb-8",
            onclick,
            {title}
        }
    }
}
