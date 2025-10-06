use crate::novel::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <NovelEditor />
        </div>
    }
}
