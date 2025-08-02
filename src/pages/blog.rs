use crate::page_loader;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        ul {
            {
                page_loader::PAGE_KEYS
                    .iter()
                    .map(|page_key| {
                        rsx!(BlogItem { page_key })
                    })
            }
        }
    }
}

#[component]
fn BlogItem(page_key: &'static page_loader::PageKey) -> Element {
    rsx! {
        li {
            a {
                href: "/blog/{page_key.short_name}",
                {page_key.title.clone()}
            }
        }
    }
}
