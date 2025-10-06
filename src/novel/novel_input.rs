use std::time::Duration;

use leptos::prelude::*;

#[component]
pub fn NovelInput(novel_text_set: WriteSignal<String>) -> impl IntoView {
    let (before_text_md5, before_text_md5_set) = signal(md5::compute(""));

    view! {
        <div class="novel-input">
            <textarea  id="novel-text-area" class="text-area"
                on:input:target = move |ev|{
                    let current_text = ev.target().value();
                    if current_text.len() < 3000{
                        *novel_text_set.write() = current_text;
                    } else{
                        let current_hash:md5::Digest = md5::compute(current_text.as_bytes());
                        *before_text_md5_set.write() = current_hash;
                        set_timeout(move ||{
                            if before_text_md5.get() == current_hash{
                                *novel_text_set.write() = current_text;
                            }
                        }, Duration::from_secs(1));
                    }
                }
        ></textarea>
        </div>
    }
}
