mod components;
mod page_loader;
mod pages;

use dioxus::prelude::*;
use pages::{Blog, Home, Links, NotFound};

use crate::components::Header;

#[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},

    #[route("/links")]
    Links {},

    // #[route("/blog/")]
    // Blog {},

    // #[route("/blog/:short_name")]
    // BlogPage { short_name: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/include/icon/favicon.ico") }

        document::Stylesheet { href: asset!("/include/styling/markdown.css") },
        document::Stylesheet { href: "/assets/katex/katex.min.css" },
        document::Stylesheet { href: asset!("/include/styling/tailwind.css") },

        Router::<Route> {}
    }
}

fn main() {
    // page_loader::load_pages();
    dioxus::launch(App);
}
