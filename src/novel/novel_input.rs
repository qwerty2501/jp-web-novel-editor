use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

#[component]
pub fn NovelInput(novel_text_set: WriteSignal<String>) -> impl IntoView {
    let (before_text_len, before_text_len_set) = signal(0);

    view! {
        <div class="novel-input">
            <div  contenteditable id="novel-text-area" class="text-area"
                on:input:target = move |ev|{
                    if let Ok(div) = ev.target().dyn_into::<HtmlDivElement>() {
                        let current_text = div.inner_text();
                        if current_text.len() < 3000{
                            *novel_text_set.write() = current_text;
                        } else{
                            *before_text_len_set.write() = current_text.len();
                            set_timeout(move ||{
                                if before_text_len.get() == current_text.len(){
                                    *novel_text_set.write() = current_text;
                                }
                            }, Duration::from_secs(1));
                        }
                    }
                }
        ></div>
        </div>
    }
}
