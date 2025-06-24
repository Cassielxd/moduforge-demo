use std::sync::Arc;

use axum::Json;
use chrono::{DateTime, Local};
use moduforge_core::types::HistoryEntryWithMeta;
use moduforge_model::{attrs::Attrs, mark::Mark, node::Node, types::NodeId};
use moduforge_rules_template::render;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, res, response::Res, ContextHelper, ResponseResult};

pub mod djgc;
pub mod fbfx_csxm;
pub mod gcxm;
pub mod rcj;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetHistoryVersionCammand {
    pub editor_name: String,
}

/// 获取历史记录
pub async fn get_history(
    Json(param): Json<GetHistoryVersionCammand>,
) -> ResponseResult<Vec<HistoryEntry>> {
    let editor = ContextHelper::get_demo_editor(&param.editor_name);
    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let editor = editor.unwrap();
    let history_manager = editor.get_history_manager();
    let history_version = history_manager.get_history();
    let history = history_version;
    let mut history_result = history
        .past
        .iter()
        .map(render_history_entry)
        .collect::<Vec<HistoryEntry>>();
    history_result.push(render_history_entry(&history.present));
    let history_future = history
        .future
        .iter()
        .map(render_history_entry)
        .collect::<Vec<HistoryEntry>>();
    history_result.extend(history_future);
    res!(history_result)
}

/// 渲染历史记录描述
fn render_history_entry(item: &HistoryEntryWithMeta) -> HistoryEntry {
    let description = render(&item.description, item.meta.clone().into()).unwrap();
    // 日期格式化成yyyy-MM-dd HH:mm:ss
    let timestamp = DateTime::<Local>::from(item.timestamp)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    return HistoryEntry {
        current: false,
        state_version: item.state.version,
        description: description.to_string(),
        timestamp: timestamp,
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 状态快照
    pub state_version: u64,

    /// 操作描述
    pub description: String,

    /// 时间戳
    pub timestamp: String,
    pub current: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GcxmTreeItem {
    pub id: NodeId,
    pub r#type: String,
    pub attrs: Attrs,
    pub children: Vec<GcxmTreeItem>,
    pub marks: Vec<Mark>,
}

impl GcxmTreeItem {
    fn from_nodes(
        root_id: NodeId,
        nodes: Vec<Arc<Node>>,
        parent_map: &im::HashMap<NodeId, NodeId>,
    ) -> Option<Self> {
        use std::collections::HashMap;
        if nodes.is_empty() {
            return None;
        }
        let node_map: HashMap<NodeId, Arc<Node>> =
            nodes.iter().map(|n| (n.id.clone(), n.clone())).collect();

        fn build_tree(
            id: &NodeId,
            node_map: &HashMap<NodeId, Arc<Node>>,
            parent_map: &im::HashMap<NodeId, NodeId>,
        ) -> Option<GcxmTreeItem> {
            let node = node_map.get(id)?;
            let children: Vec<GcxmTreeItem> = node_map
                .iter()
                .filter(|(_, n)| parent_map.get(&n.id) == Some(id))
                .filter_map(|(cid, _)| build_tree(cid, node_map, parent_map))
                .collect();
            Some(GcxmTreeItem {
                id: node.id.clone(),
                r#type: node.r#type.to_string(),
                attrs: node.attrs.clone(),
                children,
                marks: node.marks.iter().cloned().collect(),
            })
        }

        build_tree(&root_id, &node_map, parent_map)
    }
}
