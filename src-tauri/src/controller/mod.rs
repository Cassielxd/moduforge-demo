use axum::Json;
use moduforge_rules_template::render;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

use crate::{error::AppError, res,response::Res, ContextHelper, ResponseResult};

pub mod djgc;
pub mod fbfx_csxm;
pub mod gcxm;
pub mod rcj;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetHistoryVersionCammand {
    pub editor_name: String
}

/// 获取历史记录
pub async fn get_history(Json(param): Json<GetHistoryVersionCammand>) -> ResponseResult<Vec<HistoryEntry>> {
    let editor = ContextHelper::get_demo_editor(&param.editor_name);
    if editor.is_none() {
        return Err(AppError(anyhow::anyhow!("工程项目不存在".to_string())));
    }
    let  editor = editor.unwrap();
    let history_manager = editor.get_history_manager();
    let history_version = history_manager.get_history();
    let history = history_version;
    let mut history_result = history.past.iter().map(|item| {
        let description = render(&item.description,item.meta.clone().into()).unwrap();
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
    }).collect::<Vec<HistoryEntry>>();
    let description = render(&history.present.description,history.present.meta.clone().into()).unwrap();
    history_result.push(HistoryEntry {
        current: true,
        state_version: history.present.state.version,
        description: description.to_string(),
        timestamp: DateTime::<Local>::from(history.present.timestamp)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
    });

    let  history_future = history.future.iter().map(|item| {
        let description = render(&item.description,item.meta.clone().into()).unwrap();

        return HistoryEntry {
            current: false,
            state_version: item.state.version,
            description: description.to_string(),
            timestamp: DateTime::<Local>::from(item.timestamp)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        };
    }).collect::<Vec<HistoryEntry>>();
    history_result.extend(history_future);
    res!(history_result)
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
