use axum::{
    routing::{get, post},
    Router,
};

use crate::controller::gcxm::{get_gcxm_tree, insert_child, new_project};

pub fn build_app() -> Router {
    Router::new()
        //创建新工程项目
        .route("/gcxm", post(new_project))
        //插入子节点 单项、单位
        .route("/gcxm/insert_child", post(insert_child))
        //获取工程项目树
        .route("/gcxm/get_gcxm_tree/{editor_name}", get(get_gcxm_tree))
}
