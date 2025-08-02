use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div {
            class: "flex justify-center",
            div {
                class: "container mx-auto",

                {&children}
            }
        }
    }
}
