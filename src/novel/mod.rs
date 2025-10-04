mod novel_input;
mod novel_view;

pub use novel_input::*;
pub use novel_view::*;

use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct NovelText {
    text: String,
}
