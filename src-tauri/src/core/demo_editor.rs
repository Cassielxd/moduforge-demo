use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use mf_core::{async_runtime::ForgeAsyncRuntime, types::RuntimeOptions, ForgeResult};
use mf_state::{resource::Resource, resource_table::ResourceId};

pub struct DemoEditorOptions {
    pub editor_options: RuntimeOptions,
}

pub struct DemoEditor {
    /// 内部异步编辑器实例，处理底层编辑操作
    ///
    /// 负责状态管理、撤销/重做操作以及资源跟踪等基础功能
    editor: ForgeAsyncRuntime,

    /// 编辑器配置选项
    ///
    /// 包含创建和运行编辑器所需的各项配置，如存储接口和规则加载器
    options: DemoEditorOptions,
}

impl DemoEditor {
    pub async fn create(options: DemoEditorOptions) -> ForgeResult<Self> {
        // 创建异步编辑器
        let editor = ForgeAsyncRuntime::create(options.editor_options.clone()).await?;

        Ok(Self { editor, options })
    }
    pub fn get_options(&self) -> &DemoEditorOptions {
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
impl Deref for DemoEditor {
    type Target = ForgeAsyncRuntime;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for DemoEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}
