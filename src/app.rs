//use crate::menu::*;
use crate::novel::*;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        div{
            class:"app",
            NovelEditor{}
        }
    }
}
