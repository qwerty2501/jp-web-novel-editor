use leptos::prelude::*;
use reactive_stores::Store;

use crate::novel::{NovelText, NovelTextStoreFields};

#[component]
pub fn NovelView() -> impl IntoView {
    let novel_text = expect_context::<Store<NovelText>>();
    view! {
        <div class="novel-view" >
        <p class="test-view-area"> {novel_text.text()} </p>
        </div>
    }
}
