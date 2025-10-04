use leptos::prelude::*;
use reactive_stores::Store;

use crate::novel::{NovelText, NovelTextStoreFields};

#[component]
pub fn NovelInput() -> impl IntoView {
    let novel_text = expect_context::<Store<NovelText>>();
    view! {
        <div class="novel-input">
            <textarea  class="text-area"
        on:input:target = move |ev|{
            *novel_text.text().write() = ev.target().value();
        }
        ></textarea>
        </div>
    }
}
