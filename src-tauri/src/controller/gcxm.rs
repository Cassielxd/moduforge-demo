use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use axum::{extract::Path, Json};
use moduforge_core::{types::NodePoolFnTrait, ForgeResult};
use moduforge_model::{
    attrs::Attrs, id_generator::IdGenerator, mark::Mark, node::Node, node_pool::NodePool,
    types::NodeId,
};
use moduforge_state::StateConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    commands::gcxm::{AddFootNoteCammand, DeleteGcxmCammand, InsertChildCammand},
    error::AppError,
    initialize::editor::{init_editor, init_options},
    nodes::gcxm::{DWGC_STR, DXGC_STR, GCXM_STR},
    res,
    response::Res,
    ContextHelper, ResponseResult,
};

#[derive(Debug, Deserialize)]
pub struct GcxmPost {
    pub name: String,
    pub id: Option<String>,
}

impl GcxmPost {
    pub fn to_attr_map(&self) -> HashMap<String, Value> {
        let mut attr = HashMap::new();
        Self::insert_str(&mut attr, "name", &Some(self.name.clone()));
        attr
    }
    fn insert_str(map: &mut HashMap<String, Value>, key: &str, value: &Option<String>) {
        if let Some(v) = value {
            map.insert(key.to_string(), Value::String(v.clone()));
        }
    }
}

#[async_trait]
impl NodePoolFnTrait for GcxmPost {
    async fn create(&self, config: &StateConfig) -> ForgeResult<NodePool> {
        let schema = config.schema.clone().unwrap();

        let res = schema.top_node_type.clone().unwrap().create_and_fill(
            self.id.clone(),
            Some(&self.to_attr_map()),
            vec![],
            None,
            &schema,
        );
        Ok(NodePool::from(res).as_ref().clone())
    }
}

pub async fn create_editor(create_callback: Arc<GcxmPost>) -> anyhow::Result<()> {
    let option = init_options(create_callback.clone()).await;
    let editor = init_editor(option).await;
    ContextHelper::set_demo_editor(&create_callback.id.clone().unwrap(), editor);
    Ok(())
}

///创建工程项目
pub async fn new_project(Json(mut param): Json<GcxmPost>) -> ResponseResult<GcxmTreeItem> {
    let id: String = IdGenerator::get_id();
    param.id = Some(id.clone());
    create_editor(Arc::new(param)).await?;
    let editor = ContextHelper::get_demo_editor(&id).unwrap();
    let doc = editor.doc();
    let nodes: Vec<Arc<Node>> = doc.parallel_query(Box::new(|node: &Node| {
        node.r#type == DWGC_STR || node.r#type == DXGC_STR || node.r#type == GCXM_STR
    }));
    let parent_map = &doc.get_inner().parent_map;
    if let Some(root_item) = GcxmTreeItem::from_nodes(id.clone(), nodes, parent_map) {
        res!(root_item)
    } else {
        Err(AppError(anyhow::anyhow!("无法构建工程树,未找到根节点")))
    }
}
///插入子节点
pub async fn insert_child(
    Json(mut param): Json<InsertChildCammand>,
) -> ResponseResult<GcxmTreeItem> {
    let editor: Option<
        dashmap::mapref::one::RefMut<'static, String, crate::core::demo_editor::DemoEditor>,
    > = ContextHelper::get_demo_editor(&param.editor_name);

    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let mut editor = editor.unwrap();
    param.id = Some(IdGenerator::get_id());
    let meta = serde_json::to_value(param.clone())?;
    editor
        .command_with_meta(Arc::new(param.clone()), "插入 {{other.name}} 子节点".to_string(), meta)
        .await?;
    let doc = editor.doc();
    let node = doc.get_node(&param.id.clone().unwrap()).unwrap();
    let mut nodes: Vec<Arc<Node>> = doc
        .descendants(&param.id.clone().unwrap())
        .iter()
        .filter(|n| n.r#type == DWGC_STR || n.r#type == DXGC_STR)
        .cloned()
        .collect();
    nodes.push(node);
    let parent_map = &doc.get_inner().parent_map;
    if let Some(root_item) = GcxmTreeItem::from_nodes(param.id.clone().unwrap(), nodes, parent_map)
    {
        res!(root_item)
    } else {
        Err(AppError(anyhow::anyhow!(
            "无法构建工程树,未找到根节点,{}",
            param.id.unwrap()
        )))
    }
}

///获取工程项目树节点

pub async fn get_gcxm_tree(Path(editor_name): Path<String>) -> ResponseResult<GcxmTreeItem> {
    let editor = ContextHelper::get_demo_editor(&editor_name);
    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let editor = editor.unwrap();
    let doc = editor.doc();
    let nodes: Vec<Arc<Node>> = doc.parallel_query(Box::new(|node: &Node| {
        node.r#type == DWGC_STR || node.r#type == DXGC_STR || node.r#type == GCXM_STR
    }));

    let parent_map = &doc.get_inner().parent_map;
    if let Some(root_item) = GcxmTreeItem::from_nodes(editor_name, nodes, parent_map) {
        res!(root_item)
    } else {
        Err(AppError(anyhow::anyhow!("无法构建工程树,未找到根节点")))
    }
}

///删除工程项目节点
pub async fn delete_gcxm(Json(param): Json<DeleteGcxmCammand>) -> ResponseResult<String> {
    if param.id == param.editor_name {
        return Err(AppError(anyhow::anyhow!("不能删除工程项目".to_string())));
    }
    let editor = ContextHelper::get_demo_editor(&param.editor_name);
    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let mut editor = editor.unwrap();
    let node = editor.doc().get_node(&param.id).unwrap();
    let meta = serde_json::to_value(node)?;
    editor
        .command_with_meta(Arc::new(param.clone()), "删除  {{a.name}}".to_string(), meta)
        .await?;
    res!("删除成功".to_string())
}

///添加脚注
pub async fn add_footnote(Json(param): Json<AddFootNoteCammand>) -> ResponseResult<()> {
    let editor = ContextHelper::get_demo_editor(&param.editor_name);
    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let mut editor = editor.unwrap();
    let meta = serde_json::to_value(param.clone())?;
    editor
        .command_with_meta(Arc::new(param.clone()), "添加id：{{id}}脚注".to_string(), meta)
        .await?;
    res!(())
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
