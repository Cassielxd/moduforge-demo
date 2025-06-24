use std::collections::HashMap;

use async_trait::async_trait;
use moduforge_model::types::NodeId;
use moduforge_state::{transaction::Command, Transaction};
use moduforge_transform::TransformResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{commands::{DeleteNodeCammand, ShareCommand, ShareCommandData}, marks::FOOTNOTE_STR};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertChildCammand {
    pub editor_name: String,
    pub parent_id: String,
    pub id: Option<NodeId>,
    pub r#type: String,
    pub other: HashMap<String, Value>,
}

#[async_trait]
impl Command for InsertChildCammand {
    async fn execute(&self, tr: &mut Transaction) -> TransformResult<()> {
       self.add_node(tr, &ShareCommandData{
            editor_name: &self.editor_name,
            parent_id: &self.parent_id,
            id: &self.id,
            r#type: &self.r#type,
            attrs: &self.other,
        }).await
    }
    fn name(&self) -> String {
        "insert_gcxm_child".to_string()
    }
}
#[async_trait]
impl ShareCommand for InsertChildCammand {
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddFootNoteCammand {
    pub editor_name: String,
    pub id: NodeId,
    pub footnote: String,
}

#[async_trait]
impl Command for AddFootNoteCammand {
    async fn execute(&self, tr: &mut Transaction) -> TransformResult<()> {
        let mark = tr
            .schema
            .marks
            .get(FOOTNOTE_STR)
            .unwrap()
            .create(Some(&HashMap::from_iter(vec![(
                "value".to_string(),
                self.footnote.clone().into(),
            )])));
        tr.add_mark(self.id.clone(), vec![mark])?;
        Ok(())
    }
    fn name(&self) -> String {
        "add_gcxm_footnote".to_string()
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteGcxmCammand {
    pub editor_name: String,
    pub id: NodeId,
}

#[async_trait]
impl Command for DeleteGcxmCammand {
    async fn execute(&self, tr: &mut Transaction) -> TransformResult<()> {
        self.delete_node(tr, &DeleteNodeCammand{
            editor_name: &self.editor_name,
            id: &self.id,
        }).await
    }
    fn name(&self) -> String {
        "delete_gcxm".to_string()
    }
}

#[async_trait]
impl ShareCommand for DeleteGcxmCammand {
    
}