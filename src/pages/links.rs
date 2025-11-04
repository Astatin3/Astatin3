use crate::components::MarkdownPage;
use dioxus::prelude::*;
// use manganis::*;

// const INDEX_PAGE: &[u8] = mg!(file("/pages/index.md").bytes());

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Links() -> Element {
    // let content = INDEX_PAGE.to_vec();
    rsx! {
        div {
            MarkdownPage { content: include_str!("../../tabs/links.md") }
        }
    }
}
