use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};

use mf_collab_client::{
    mapping::Mapper, provider::WebsocketProvider, sync::Awareness, types::SyncEvent, utils::Utils,
    AwarenessRef, Doc,
};
use mf_core::{
    async_runtime::ForgeAsyncRuntime,
    error_utils,
    types::{Content, RuntimeOptions},
    ForgeResult,
};
use mf_model::node_pool::NodePool;
use mf_state::{resource::Resource, resource_table::ResourceId};

pub struct CollabEditorOptions {
    pub editor_options: RuntimeOptions,
    pub server_url: String,
    pub room_name: String,
}
impl CollabEditorOptions {
    pub fn new(server_url: String, room_name: String) -> Self {
        Self {
            editor_options: RuntimeOptions::default(),
            server_url,
            room_name,
        }
    }
}

pub struct CollabEditor {
    /// 内部异步编辑器实例，处理底层编辑操作
    ///
    /// 负责状态管理、撤销/重做操作以及资源跟踪等基础功能
    editor: ForgeAsyncRuntime,
    /// 协作编辑器提供者
    ///
    pub sync_manager: CollabSyncManager,

    /// 编辑器配置选项
    ///
    /// 包含创建和运行编辑器所需的各项配置，如存储接口和规则加载器
    options: CollabEditorOptions,
}

impl CollabEditor {
    pub async fn create(options: CollabEditorOptions) -> ForgeResult<Self> {
        let mut sync_manager = CollabSyncManager::new(&options).await?;
        let mut event_rx = sync_manager.provider.subscribe_sync_events().unwrap();
        sync_manager.provider.connect().await;
        // 设置超时 10 秒
        let timeout = tokio::time::timeout(Duration::from_secs(10), async {
            while let Ok(event) = event_rx.recv().await {
                if let SyncEvent::InitialSyncCompleted { has_data, .. } = event {
                    return Ok(has_data);
                }
            }
            Err(error_utils::event_error("服务器异常，请检查服务器是否启动"))
        })
        .await?;
        let editor = match timeout {
            Ok(true) => {
                //有数据 反向同步数据 并创建 编辑器
                let awareness_lock = sync_manager.awareness.read().await;
                //把 远程的 文档 转换成 树
                match Mapper::yrs_doc_to_tree(awareness_lock.doc()) {
                    Ok(tree) => {
                        //转换成功
                        let pool = NodePool::new(Arc::new(tree)).as_ref().clone();
                        let options = options
                            .editor_options
                            .clone()
                            .set_content(Content::NodePool(pool));
                        ForgeAsyncRuntime::create(options).await?
                    }
                    Err(_) => {
                        //转换失败 创建一个本地编辑器 并同步数据到远程
                        let ed = ForgeAsyncRuntime::create(options.editor_options.clone()).await?;
                        sync_manager.sync_to_remote(&ed).await?;
                        ed
                    }
                }
            }
            Ok(false) => {
                //没有数据 创建一个本地编辑器 并同步数据到远程
                let ed = ForgeAsyncRuntime::create(options.editor_options.clone()).await?;
                sync_manager.sync_to_remote(&ed).await?;
                ed
            }
            _ => {
                return Err(anyhow::anyhow!("服务器异常，请检查服务器是否启动"));
            }
        };

        Ok(Self {
            editor,
            options,
            sync_manager,
        })
    }

    pub fn get_options(&self) -> &CollabEditorOptions {
        &self.options
    }

    pub fn get_resource<T: Resource>(&self, rid: ResourceId) -> Option<Arc<T>> {
        // 获取编辑器状态
        // 编辑器状态包含所有运行时资源和配置
        let state = self.editor.get_state();

        // 获取资源管理器
        // 资源管理器负责管理系统中所有注册的资源
        let resource_manager = state.resource_manager();

        // 从资源表中获取指定类型和ID的资源
        // 如果资源不存在或类型不匹配，将返回错误
        resource_manager.resource_table.get::<T>(rid)
    }
}
impl Deref for CollabEditor {
    type Target = ForgeAsyncRuntime;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for CollabEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}

pub struct CollabSyncManager {
    provider: WebsocketProvider,
    awareness: AwarenessRef,
    // ... 其他同步相关字段
}

impl CollabSyncManager {
    pub async fn new(options: &CollabEditorOptions) -> ForgeResult<Self> {
        let doc = Doc::new();
        let awareness: AwarenessRef = Arc::new(tokio::sync::RwLock::new(Awareness::new(doc)));
        Ok(Self {
            provider: WebsocketProvider::new(
                options.server_url.to_string(),
                options.room_name.to_string(),
                awareness.clone(),
            )
            .await,
            awareness,
        })
    }
    pub async fn sync_to_remote(&self, editor: &ForgeAsyncRuntime) -> ForgeResult<()> {
        let doc = editor.get_state().doc();
        let tree = doc.get_inner().as_ref();
        Utils::apply_tree_to_yrs(self.awareness.clone(), tree).await?;
        Ok(())
    }
}
