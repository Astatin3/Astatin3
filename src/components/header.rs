use dioxus::prelude::*;

use crate::{components::container::Container, Route};

pub struct Link {
    pub href: String,
    pub text: String,
}

#[component]
pub fn Vseperator() -> Element {
    rsx! {
        div {
            class: "separator",
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        Container {
            div {
                class: "header relative",

                span {
                    class: "title",
                    "ASTATIN3"
                },

                div {
                    class: "absolute inset-y-0 right-0",
                    a {
                        href: "/",
                        "HOME"
                    },

                    Vseperator {},

                    a {
                        href: "/blog/",
                        "BLOG"
                    },

                    Vseperator {},

                    a {
                        href: "/test/",
                        "TEST"
                    }
                }
            }

            Outlet::<Route> {}
        }

    }
}
