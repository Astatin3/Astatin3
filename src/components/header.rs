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
                        "Home"
                    },

                    Vseperator {},

                    a {
                        href: "/schedule/",
                        "Schedule"
                    },

                    Vseperator {},

                    a {
                        href: "/rustex/",
                        "RusTeX"
                    },

                    Vseperator {},

                    a {
                        href: "/test/",
                        "Test Page"
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
