mod components;
mod page_loader;
mod pages;

use dioxus::prelude::*;
use pages::BlogPage;
use pages::NotFound;

use crate::components::{Header, MarkdownPage};

#[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},

    #[route("/schedule")]
    Schedule {},

    #[route("/rustex")]
    RusTeX {},

    #[route("/test")]
    TestPage {},

    // #[route("/notes")]
    // Notes {},

    // #[route("/blog/")]
    // Blog {},
    #[route("/blog/:short_name")]
    BlogPage { short_name: String },

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

#[component]
pub fn Home() -> Element {
    // let content = INDEX_PAGE.to_vec();
    rsx! {
        div {
            MarkdownPage { content: include_str!("../tabs/home.md") }
        }
    }
}

#[component]
pub fn Schedule() -> Element {
    // let content = INDEX_PAGE.to_vec();
    rsx! {
        div {
            MarkdownPage { content: include_str!("../tabs/schedule.md") }
        }
    }
}

#[component]
pub fn RusTeX() -> Element {
    // let content = INDEX_PAGE.to_vec();
    rsx! {
        div {
            MarkdownPage { content: include_str!("../tabs/rustex.md") }
        }
    }
}

#[component]
pub fn TestPage() -> Element {
    // let content = INDEX_PAGE.to_vec();
    rsx! {
        div {
            MarkdownPage { content: include_str!("../tabs/test.md") }
        }
    }
}
