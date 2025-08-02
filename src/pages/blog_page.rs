use crate::{page_loader, pages::NotFound};
use dioxus::prelude::*;

use crate::components::MarkdownPage;

#[component]
pub fn BlogPage(short_name: String) -> Element {
    match page_loader::PAGE_DATA.get(&short_name) {
        Some(content) => rsx! {
            MarkdownPage {
                content: content.content.clone(),
            }
        },
        None => rsx! {
            NotFound { route: vec![] }
        },
    }
}
