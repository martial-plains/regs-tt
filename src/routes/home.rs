use dioxus::prelude::*;

use crate::{components::header::Header, routes::Route};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "bg-mobile-light bg-lightGrey md:bg-tablet-light xl:bg-desktop-light dark:bg-mobile-dark dark:bg-darkNavy dark:md:bg-tablet-dark dark:xl:bg-desktop-dark min-h-screen bg-no-repeat xl:bg-cover",

            Header {}

            main { class: "mt-8 mx-6 md:mx-16 xl:mx-[140px]", Outlet::<Route> {} }
        }
    }
}
