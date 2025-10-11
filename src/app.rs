//use crate::menu::*;
use crate::app::document::Stylesheet;
use crate::novel::*;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Stylesheet { href: asset!("assets/main.scss") },
        div{
            class:"app",
            NovelEditor{}
        }
    }
}
