#![warn(clippy::pedantic, clippy::nursery)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use routes::Route;
use types::Quiz;

mod components;
mod routes;
mod types;

const STYLE: &str = asset!("assets/styles/tailwind/tailwind.css");

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(target_arch = "wasm32")]
    {
        launch(app);
    }
}

fn app() -> Element {
    use_context_provider(|| Signal::new(Quiz::default()));
    use_context_provider(|| Signal::new(0_i32));

    rsx! {
        head::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
