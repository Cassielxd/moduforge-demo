use std::collections::HashMap;

use async_trait::async_trait;
use moduforge_model::types::NodeId;
use moduforge_state::{transaction::Command, Transaction};
use moduforge_transform::TransformResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod djgc;
pub mod fbfx_csxm;
pub mod gcxm;
pub mod rcj;


#[derive(Debug, Serialize, Clone)]
pub struct ShareCommandData<'a> {
    pub editor_name: &'a str,
    pub parent_id: &'a str,
    pub id: &'a Option<NodeId>,
    pub r#type: &'a str,
    pub attrs: &'a HashMap<String, Value>,
}
#[derive(Debug, Serialize, Clone)]
pub struct DeleteNodeCammand<'a> {
    pub editor_name: &'a str,
    pub id: &'a NodeId,
}


#[async_trait]
pub trait ShareCommand: Command {
    async fn add_node(&self, tr: &mut Transaction, data: &ShareCommandData<'_>) -> TransformResult<()>{
        if tr.doc().get_node(&data.parent_id.to_string()).is_none() {
            return Err(anyhow::anyhow!("目标节点不存在".to_string()));
        }
        if let Some(node_type) = tr.schema.nodes.get(data.r#type) {
            let nodes = node_type.create_and_fill(data.id.clone(), Some(&data.attrs), vec![], None, &tr.schema);
            tr.add_node(data.parent_id.to_string(), vec![nodes])?;
        } else {
            return Err(anyhow::anyhow!("节点类型不存在".to_string()));
        }
        Ok(())
    }
    async fn delete_node(&self, tr: &mut Transaction, data: &DeleteNodeCammand) -> TransformResult<()>{
       //组装参数 前置必要操作
        //获取目标节点
        let _point_node = {
            match tr.doc().get_node(&data.id) {
                Some(n) => n,
                None => {
                    return Err(anyhow::anyhow!("目标节点不存在".to_string()));
                }
            }
        };
        let parent_id = tr.doc().get_parent_node(&data.id).unwrap().id.clone();
        tr.remove_node(parent_id, vec![data.id.clone()])?;
        Ok(())
    }

}