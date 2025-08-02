use crate::components::markdown::Markdown;
use dioxus::prelude::*;
use pulldown_cmark::Options;

// use crate::components::pulldown_cmark::Markdown;
// use pulldown_cmark::TextMergeStream;

// use crate::markdown::Markdown;

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
pub fn MarkdownPage(content: String) -> Element {
    rsx! {
        // Container {
        //     Header {},

            Markdown {
                src: content,
                theme: "base16-ocean.dark",
                parse_options: Options::ENABLE_GFM
                    | Options::ENABLE_DEFINITION_LIST
                    | Options::ENABLE_TABLES
                    | Options::ENABLE_TASKLISTS
                    | Options::ENABLE_WIKILINKS
                    | Options::ENABLE_STRIKETHROUGH
                    | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
                    | Options::ENABLE_SMART_PUNCTUATION
                    | Options::ENABLE_MATH
                    | Options::ENABLE_HEADING_ATTRIBUTES,
            }
        // }
    }
}
