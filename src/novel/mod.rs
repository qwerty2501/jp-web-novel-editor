mod novel_input;
mod novel_view;

use novel_input::*;
use novel_view::*;

use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct NovelText {
    text: String,
}

#[component]
pub fn NovelEditor() -> impl IntoView {
    let (novel_text, novel_text_set) = signal(String::new());
    view! {
        <div class="novel-editor" >
            <NovelInput novel_text_set=novel_text_set />
            <NovelView  novel_text=novel_text/>
        </div>

    }
}
