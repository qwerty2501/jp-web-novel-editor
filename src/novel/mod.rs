mod novel_input;
mod novel_view;

use novel_input::*;
use novel_view::*;

use dioxus::prelude::*;

#[component]
pub fn NovelEditor() -> Element {
    rsx! {
        div{
            class:"novel-editor",
            NovelInput{},
            NovelView{},
        }
    }
}
