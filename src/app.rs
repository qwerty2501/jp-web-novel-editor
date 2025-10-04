use crate::novel::NovelInput;
use crate::novel::NovelText;
use crate::novel::NovelView;
use leptos::prelude::*;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(NovelText::default()));
    view! {
        <div class="app">
            <NovelInput/>
            <NovelView/>
        </div>
    }
}
