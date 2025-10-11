use crate::menu::*;
use crate::novel::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <Menu />
            <NovelEditor />
        </div>
    }
}
