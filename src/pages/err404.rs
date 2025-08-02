use dioxus::prelude::*;

use crate::components::MarkdownPage;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        MarkdownPage { content: include_str!("../../pages/404.md") }
    }
}
