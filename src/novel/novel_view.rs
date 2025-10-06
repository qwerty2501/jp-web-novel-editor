use jp_web_novel_text::{
    DictionaryPhrase, DictionaryWord, NewLinePhrase, Parser, Phrase, PlainPhrase, RubyPhrase,
    WhiteSpacePhrase, WhiteSpaceType,
};
use leptos::prelude::*;
use reactive_stores::Store;

use crate::novel::{NovelText, NovelTextStoreFields};

#[component]
pub fn NovelView(novel_text: ReadSignal<String>) -> impl IntoView {
    let parser = Parser::default();
    view! {
        <div class="novel-view" >
            <div class="test-view-area">
            {move || render_phrases(&parser,&novel_text.get())}
            </div>
        </div>
    }
}

pub fn render_phrases(parser: &Parser, input: &str) -> impl IntoView + use<> {
    parser
        .parse_iter(input)
        .map(|frag| match frag.phrase() {
            Phrase::Plain(phrase) => render_plain(phrase).into_any(),
            Phrase::Ruby(phrase) => render_ruby(phrase).into_any(),
            Phrase::NewLine(phrase) => render_new_line(phrase).into_any(),
            Phrase::WhiteSpace(phrase) => render_white_space(phrase).into_any(),
            Phrase::DictionaryWord(phrase) => render_dictionary_word(phrase).into_any(),
        })
        .collect_view()
}

fn render_plain(phrase: &PlainPhrase<&str>) -> impl IntoView {
    view! {
        <span>{phrase.target().to_string()}</span>
    }
}

fn render_ruby(phrase: &RubyPhrase<&str>) -> impl IntoView {
    view! {
        <ruby>{phrase.target().to_string()}<rp>{"("}</rp><rt>{phrase.ruby().to_string()}</rt><rp>{")"}</rp></ruby>
    }
}

fn render_new_line(_: &NewLinePhrase) -> impl IntoView {
    view! {
        <br/>
    }
}

fn render_white_space(phrase: &WhiteSpacePhrase) -> impl IntoView {
    view! {
        <span style=
        {
           "margin-left:".to_string() + &( match phrase.white_space_type(){
                WhiteSpaceType::Space => 1,
                WhiteSpaceType::ZenkakuSpace => 2,
                WhiteSpaceType::Tab => 4,
            } * phrase.count() * 10
           ).to_string() + "px"
        }
        >
        </span>
    }
}

fn render_dictionary_word(phrase: &DictionaryPhrase<&str, &DictionaryWord>) -> impl IntoView {
    view! {
        <ruby>{phrase.target().to_string()}<rp>{"("}</rp><rt>{""}</rt><rp>{")"}</rp></ruby>
    }
}
