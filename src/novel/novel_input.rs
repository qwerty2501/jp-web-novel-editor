use std::time::Duration;

use leptos::prelude::*;
use reactive_stores::Store;

use crate::novel::{NovelText, NovelTextStoreFields};

#[component]
pub fn NovelInput() -> impl IntoView {
    let novel_text = expect_context::<Store<NovelText>>();
    let (before_text_md5, before_text_md5_set) = signal(md5::compute(""));

    view! {
        <div class="novel-input">
            <textarea  id="novel-text-area" class="text-area"
                on:input:target = move |ev|{
                    if ev.target().value().len() < 3000{
                        *novel_text.text().write() = ev.target().value();
                    } else{
                        let current_hash:md5::Digest = md5::compute(ev.target().value());
                        *before_text_md5_set.write() = current_hash;
                        set_timeout(move ||{
                            if before_text_md5.get() == current_hash{
                                *novel_text.text().write() = ev.target().value();
                            }
                        }, Duration::from_secs(1));
                    }
                }
        ></textarea>
        </div>
    }
}
