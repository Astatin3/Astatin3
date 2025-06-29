mod components;
#[allow(dead_code)]
mod markdown;
mod views;

use dioxus::prelude::*;
use views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: usize },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/icon/favicon.ico") }
        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/main.css") }
        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/colors.css") }


        document::Stylesheet { href: asset!("/assets/tailwind.css") }


        // document::Link { rel: "stylesheet", href: asset!("/assets/styling/markdown_theme_light.css") }
        // script { src: "https://cdn.tailwindcss.com" }

        // document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss/dist/tailwind.min.css" }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
