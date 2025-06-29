// use crate::dioxus_elements::script::text;
use std::fs;

use dioxus::prelude::*;

use crate::markdown::Markdown;
// use crate::Markdown;

// const TEST: &str = include_str!("../../pages/index.md");

#[component]
pub fn SideLink(location: String, link_text: String) -> Element {
    rsx! {
        a {
            class: "nav-link",
            href: location,
            dangerous_inner_html: link_text,
        }
        br {  }
    }
}

#[component]
pub fn MarkdownPage(src: String) -> Element {
    rsx! {

        // div {
        //     class: "sidebar-left"
        // }
        div {
            class: "flex justify-center",
            div {
                class: "container mx-auto",

                // div {
                //     id: "left-nav",
                //     class: "sidebar-left nav",
                //     SideLink {
                //         location: "/".to_string(),
                //         link_text: "astatin3".to_string(),
                //     },
                //     SideLink {
                //         location: "/".to_string(),
                //         link_text: "Home".to_string(),
                //     }
                // }
                // div {
                //     id: "right-nav",
                //     class: "sidebar-right nav",
                // }
                Markdown {
                    src: fs::read_to_string(src).unwrap(),
                    wikilinks: true,
                    // hard_line_breaks: true,
                    theme: "base16-eighties.dark",
                }

            }
        }
        // div {
        //     class: "sidebar-right"
        // }
    }
}
