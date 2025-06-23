use std::sync::Arc;

use moduforge_core::types::{Content, EditorOptionsBuilder, Extensions, NodePoolFnTrait};

use crate::{
    core::demo_editor::{DemoEditor, DemoEditorOptions},
    marks,
    nodes::{
        fbfx_csxm::{init_fbfx_csxm_fields, CSXM_STR, DE_STR, FBFX_STR},
        gcxm::{init_project_structure, DWGC_STR},
        rcj::{init_rcj_fields, RCJ_STR},
    },
};
//获取编辑器
pub async fn init_editor(options: DemoEditorOptions) -> DemoEditor {
    DemoEditor::create(options).await.unwrap()
}
//获取编辑器配置
pub async fn init_options(create_callback: Arc<dyn NodePoolFnTrait>) -> DemoEditorOptions {
    let mut builder = EditorOptionsBuilder::new();
    builder = builder
        .content(Content::NodePoolFn(create_callback))
        .extensions(init_extension());
    let options = builder.build();
    DemoEditorOptions {
        editor_options: options,
    }
}
//获取扩展
pub fn init_extension() -> Vec<Extensions> {
    let mut extensions = vec![
        Extensions::M(marks::BG_COLOR.clone()),
        Extensions::M(marks::FOOTNOTE.clone()),
    ];
    // 工程项目、单项、单位Node
    let nodes = init_project_structure();
    for mut node in nodes {
        if node.get_name() == DWGC_STR {
            node.set_content(&format!("{}|{}+", FBFX_STR, CSXM_STR));
        }
        extensions.push(Extensions::N(node));
    }
    let fbfx_csxm_nodes = init_fbfx_csxm_fields();
    for mut node in fbfx_csxm_nodes {
        if node.get_name() == DE_STR {
            node.set_content(&format!("{}*", RCJ_STR));
        }
        extensions.push(Extensions::N(node));
    }
    // 定额下人材机明细Node
    let rcj_node = init_rcj_fields();
    extensions.push(Extensions::N(rcj_node));
    extensions
}
