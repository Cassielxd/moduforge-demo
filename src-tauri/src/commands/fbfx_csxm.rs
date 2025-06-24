use std::collections::HashMap;

use async_trait::async_trait;
use moduforge_model::types::NodeId;
use moduforge_state::{transaction::Command, Transaction};
use moduforge_transform::TransformResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::commands::{ShareCommand, ShareCommandData};

// 插入分部分项
#[derive(Debug, Serialize, Deserialize, Clone)]
struct InsertFbfxCsxmCommand {
    pub editor_name: String,
    pub parent_id: String,
    pub id: Option<NodeId>,
    pub r#type: String,
    pub attrs: HashMap<String, Value>,
}

#[async_trait]
impl Command for InsertFbfxCsxmCommand {
    async fn execute(&self, tr: &mut Transaction) -> TransformResult<()> {
        self.add_node(tr, &ShareCommandData{
            editor_name: &self.editor_name,
            parent_id: &self.parent_id,
            id: &self.id,
            r#type: &self.r#type,
            attrs: &self.attrs,
        }).await
    }
    
    fn name(&self) -> String {
        "insert_fbfx_csxm".to_string()
    }
}

#[async_trait]
impl ShareCommand for InsertFbfxCsxmCommand {
    
}