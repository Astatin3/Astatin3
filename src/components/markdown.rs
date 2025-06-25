use std::fs;

use dioxus::prelude::*;

use crate::markdown::Markdown;
// use crate::Markdown;

// const TEST: &str = include_str!("../../pages/index.md");

#[component]
pub fn MarkdownPage(src: String) -> Element {
    rsx! {

        // script { src: asset!("/assets/js/markdown.js") }
        div {
            class: "container",

            Markdown {
                src: fs::read_to_string(src).unwrap(),
                wikilinks: true,
                // hard_line_breaks: true,
                theme: "base16-ocean.dark",
            }

        }
    }
}
