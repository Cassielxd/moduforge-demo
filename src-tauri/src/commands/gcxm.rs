use std::collections::HashMap;

use async_trait::async_trait;
use moduforge_model::types::NodeId;
use moduforge_state::{transaction::Command, Transaction};
use moduforge_transform::TransformResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertChildCammand {
    pub editor_name: String,
    pub parent_id: String,
    pub id: Option<NodeId>,
    pub name: String,
    pub r#type: String,
    pub other: HashMap<String, Value>,
}

#[async_trait]
impl Command for InsertChildCammand {
    async fn execute(&self, tr: &mut Transaction) -> TransformResult<()> {
        //组装参数 前置必要操作
        //获取目标节点
        let point_node = {
            match tr.doc().get_node(&self.parent_id) {
                Some(n) => n,
                None => {
                    return Err(anyhow::anyhow!("目标节点不存在".to_string()));
                }
            }
        };
        if let Some(node_type) = tr.schema.nodes.get(&self.r#type) {
            let nodes =
                node_type.create_and_fill(self.id.clone(), Some(&self.other), vec![], None, &tr.schema);
            tr.add_node(point_node.id.to_string(), vec![nodes])?;
        } else {
            return Err(anyhow::anyhow!("type参数节点类型不存在".to_string()));
        }
        Ok(())
    }
    fn name(&self) -> String {
        "insert_gcxm_child".to_string()
    }
}
