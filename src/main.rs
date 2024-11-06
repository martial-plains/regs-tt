#![warn(clippy::pedantic, clippy::nursery)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use routes::Route;
use types::Quiz;

mod components;
mod routes;
mod types;

const STYLE: Asset = asset!("./assets/styles/tailwind/tailwind.css");

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        let cfg = dioxus::desktop::Config::new().with_window(
            WindowBuilder::new()
                .with_title("Regs TT")
                .with_always_on_top(false)
                .with_min_inner_size(LogicalSize::new(400, 600)),
        );
        LaunchBuilder::desktop().with_cfg(cfg).launch(app);
    }

    #[cfg(feature = "web")]
    {
        launch(app);
    }
}

fn app() -> Element {
    use_context_provider(|| Signal::new(Quiz::default()));
    use_context_provider(|| Signal::new(0_i32));

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
