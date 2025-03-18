use dioxus::prelude::*;

use crate::ui::components::Header;

const HOME_CSS: Asset = asset!("/assets/styling/views/home.css");

#[component]
pub fn Home<R>() -> Element
where
    R: Routable + Clone,
{
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }


        document::Link { r#type: "stylesheet" }
        div { id: "home-container",

            Header {}

            main { id: "main-content", Outlet::<R> {} }
        }
    }
}
