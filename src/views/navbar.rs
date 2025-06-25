use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn border_link(link: Route, content: &'static str) -> Element {
    rsx! {
        Link {
            to: link,
            class: "border-link",
            span {
                dangerous_inner_html: content
            }
        }
    }
}

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const NAVBAR_ICON: Asset = asset!("/assets/icon/icon.png");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            img { src: NAVBAR_ICON },
            border_link {
                link: Route::Home {},
                content: "Home"
            }
            border_link {
                link: Route::Blog { id: 1 },
                content: "Blog"
            }

            // #[layout(Navbar)]
            // border_link(Route::Home {}, "Home"),
            // border_link(Route::Blog { id: 1 }, "Blog"),


            // Link {
            //     to: Route::Home {},
            //     "Home"
            // }
            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Blog"
            // }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
