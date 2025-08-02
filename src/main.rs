mod components;
// #[allow(dead_code)]
// mod markdown;
mod views;

use dioxus::prelude::*;
use views::{Blog, Home};

use crate::components::Header;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Header)]
    #[route("/")]
    Home {},
    #[route("/blog/")]
    Blog {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/include/icon/favicon.ico") }

        document::Stylesheet { href: asset!("/include/styling/markdown.css") },
        document::Stylesheet { href: "/assets/katex/katex.min.css" },
        document::Stylesheet { href: asset!("/include/styling/tailwind.css") },
        // document::Script { src: asset!("/assets/scripts/katex.min.js") }

        // document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/katex/dist/katex.min.css" }

        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/main.css") }
        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/colors.css") }


        // document::Stylesheet { href: asset!("/assets/styling/katex.min.css") }


        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/markdown_theme_light.css") }
        // script { src: "https://cdn.tailwindcss.com" }

        // document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss/dist/tailwind.min.css" }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
