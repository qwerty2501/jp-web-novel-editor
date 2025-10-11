mod novel_input;
mod novel_view;

use novel_input::*;
use novel_view::*;

use dioxus::prelude::*;

#[component]
pub fn NovelEditor() -> Element {
    let novel_text = use_signal(|| String::new());
    rsx! {
        div{
            class:"novel-editor",
            NovelInput{
                novel_text_set:novel_text,
            },
            NovelView{
                novel_text:novel_text,
            },
        }
    }
}
