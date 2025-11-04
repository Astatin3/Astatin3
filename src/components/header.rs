use dioxus::prelude::*;

use crate::{Route, components::container::Container};

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
                        href: "/links/",
                        "LINKS"
                    },

                    // Vseperator {},

                    // a {
                    //     href: "/",
                    //     "TEST"
                    // }
                }
            }

            Outlet::<Route> {}
        }

    }
}
