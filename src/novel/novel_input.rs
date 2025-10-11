use dioxus::{prelude::*, web::WebEventExt};
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
#[derive(PartialEq, Clone, Props)]
pub struct NovelInputProps {
    novel_text_set: Signal<String>,
}

#[component]
pub fn NovelInput(mut props: NovelInputProps) -> Element {
    let mut text_area = use_signal(|| None);
    let mut before_text_len = use_signal(|| 0);

    rsx! {
        div{
            class:"novel-input",
            div{
                contenteditable:true,
                id:"novel-text-area",
                class:"text-area",
                onmounted: move |element|{
                    text_area.set(Some(element));
                },
                oninput: move |_| {
                    if let Some(text_area) = text_area.cloned(){
                        let element = text_area.as_web_event();
                        let element = element.dyn_into::<HtmlElement>().unwrap();

                        let current_text = element.inner_text();
                        if current_text.len() < 3000{
                            props.novel_text_set.set(current_text);
                        } else{
                            *before_text_len.write() = current_text.len();
                            let timeout = Timeout::new(1000, move ||{
                                if *before_text_len.read() == current_text.len() {
                                    props.novel_text_set.set(current_text);
                                }
                            });
                            timeout.forget();
                        }
                    }
                },
            }
        }
    }
}
