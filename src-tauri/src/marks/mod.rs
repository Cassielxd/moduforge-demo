use lazy_static::lazy_static;
use moduforge_core::mark::Mark;
use moduforge_macros::mark;
pub const BG_COLOR_STR: &str = "bgColor";
pub const FOOTNOTE_STR: &str = "footnote";

lazy_static! {
    pub static ref BG_COLOR: Mark = mark!(BG_COLOR_STR, "背景颜色","value"=>"#ffffff".into());
    pub static ref FOOTNOTE: Mark = mark!(FOOTNOTE_STR, "脚注","value"=>"".into());
}
