use dashmap::DashMap;

use crate::{core::demo_editor::DemoEditor, ContextHelper};

pub mod editor;

pub async fn init_contex() {
    let map_p: DashMap<String, DemoEditor> = DashMap::new();
    ContextHelper::set(map_p);
}
