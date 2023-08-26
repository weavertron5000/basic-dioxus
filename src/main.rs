#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx : Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "text-2xl text-blue-900 font-bold text-center w-full",
            "Hello world!"
        }
    })
}
