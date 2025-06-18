<script lang="ts">
import { defineComponent, ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import RightTablePanel, { TableColumn } from "../components/RightTablePanel.vue";

interface TreeTableData {
  id: number | string;
  name: string;
  type: string;
  size?: string;
  date: string;
  children?: TreeTableData[];
}

export default defineComponent({
  name: "FbfxView",
  components: { RightTablePanel },
  setup(props, { expose }) {
    const tableTreeData = ref<TreeTableData[]>([
      {
        id: 1,
        name: "文档目录",
        type: "folder",
        date: "2024-07-29",
        children: [
          {
            id: 11,
            name: "项目说明.docx",
            type: "file",
            size: "2.5MB",
            date: "2024-07-29",
          },
          {
            id: 12,
            name: "需求文档.pdf",
            type: "file",
            size: "1.8MB",
            date: "2024-07-28",
          },
        ],
      },
      {
        id: 2,
        name: "代码目录",
        type: "folder",
        date: "2024-07-29",
        children: [
          {
            id: 21,
            name: "src",
            type: "folder",
            date: "2024-07-29",
            children: [
              {
                id: 211,
                name: "main.ts",
                type: "file",
                size: "1.2KB",
                date: "2024-07-29",
              },
              {
                id: 212,
                name: "App.vue",
                type: "file",
                size: "3.4KB",
                date: "2024-07-29",
              },
            ],
          },
          {
            id: 22,
            name: "package.json",
            type: "file",
            size: "2.1KB",
            date: "2024-07-28",
          },
        ],
      },
      {
        id: 3,
        name: "readme.md",
        type: "file",
        size: "5.2KB",
        date: "2024-07-27",
      },
    ]);

    const tableColumns: TableColumn[] = [
      { prop: "name", label: "名称", width: 200 },
      { prop: "type", label: "类型", width: 100 },
      { prop: "size", label: "大小", width: 100 },
      { prop: "date", label: "修改时间", width: 150 },
    ];

    // 历史记录表格列定义
    const historyColumns: TableColumn[] = [
      { prop: "time", label: "时间", width: 150 },
      { prop: "action", label: "操作", width: 80 },
      { prop: "file", label: "文件", minWidth: 200 },
      { prop: "user", label: "用户", width: 100 },
    ];

    // 详细信息表格列定义
    const detailColumns: TableColumn[] = [
      { prop: "property", label: "属性", width: 120 },
      { prop: "value", label: "值", minWidth: 200 },
    ];

    // 统计信息表格列定义
    const statisticsColumns: TableColumn[] = [
      { prop: "name", label: "统计项", width: 120 },
      { prop: "value", label: "数值", width: 80, align: "right" },
      { prop: "unit", label: "单位", width: 60 },
      { prop: "description", label: "说明", minWidth: 200 },
    ];

    // 子标签页状态
    const activeSubTab = ref("detail");

    // 历史记录数据
    const historyData = ref([
      {
        id: 1,
        time: "2024-07-29 15:30",
        action: "创建",
        file: "项目说明.docx",
        user: "张三",
      },
      {
        id: 2,
        time: "2024-07-29 14:20",
        action: "修改",
        file: "需求文档.pdf",
        user: "李四",
      },
      {
        id: 3,
        time: "2024-07-29 13:15",
        action: "删除",
        file: "临时文件.tmp",
        user: "王五",
      },
      {
        id: 4,
        time: "2024-07-28 16:45",
        action: "创建",
        file: "代码目录",
        user: "赵六",
      },
    ]);

    // 详细信息数据（转换为表格格式）
    const detailData = ref([
      { id: 1, property: "文件类型", value: "目录/文件" },
      { id: 2, property: "创建时间", value: "2024-07-29" },
      { id: 3, property: "修改时间", value: "2024-07-29" },
      { id: 4, property: "文件大小", value: "2.5MB" },
      { id: 5, property: "权限", value: "读写执行" },
      { id: 6, property: "所有者", value: "当前用户" },
    ]);

    // 统计信息数据（转换为表格格式）
    const statisticsData = ref([
      {
        id: 1,
        name: "总文件数",
        value: "156",
        unit: "个",
        description: "包含所有文件和文件夹",
      },
      { id: 2, name: "文件夹数", value: "23", unit: "个", description: "目录文件夹数量" },
      { id: 3, name: "总大小", value: "45.2", unit: "MB", description: "所有文件总大小" },
      {
        id: 4,
        name: "最近修改",
        value: "12",
        unit: "个",
        description: "近7天内修改的文件",
      },
    ]);

    const init = (id: string | number | null) => {
      console.log("FbfxView.vue init called with ID:", id);
      // 根据ID可以刷新不同的文件数据
    };

    // 处理子标签页切换
    const handleSubTabChange = (tabName: string | number) => {
      console.log("FbfxView: Sub tab changed to:", tabName);
      // 可以根据不同的子标签页加载不同的数据
    };

    const rightTablePanelRef = ref();

    // 处理表格事件
    const handleAddRow = (currentRow?: TreeTableData) => {
      const newRow: TreeTableData = {
        id: Date.now(),
        name: "新文件",
        type: "file",
        size: "0KB",
        date: new Date().toISOString().split("T")[0],
      };

      if (currentRow) {
        // 在指定行的下一行插入新行
        const insertAfter = (
          data: TreeTableData[],
          targetId: number | string
        ): boolean => {
          for (let i = 0; i < data.length; i++) {
            if (data[i].id === targetId) {
              // 在当前位置的下一行插入
              data.splice(i + 1, 0, newRow);
              return true;
            }
            // 如果有子节点，递归查找
            if (data[i].children && insertAfter(data[i].children!, targetId)) {
              return true;
            }
          }
          return false;
        };

        if (!insertAfter(tableTreeData.value, currentRow.id)) {
          // 如果没有找到指定行，就添加到末尾
          tableTreeData.value.push(newRow);
        }
        ElMessage.success("在选中行下方添加成功");
      } else {
        // 没有指定行时，添加到末尾
        tableTreeData.value.push(newRow);
        ElMessage.success("添加成功");
      }

      // 选中新添加的行
      if (rightTablePanelRef.value) {
        rightTablePanelRef.value.setCurrentRow(newRow.id);
      }
    };

    const handleAddChild = (parentRow: TreeTableData) => {
      const newChild: TreeTableData = {
        id: Date.now(),
        name: "新子项",
        type: "file",
        size: "0KB",
        date: new Date().toISOString().split("T")[0],
      };

      if (!parentRow.children) {
        parentRow.children = [];
      }
      parentRow.children.push(newChild);
      ElMessage.success("添加子项成功");

      // 展开父节点以显示新添加的子项
      if (rightTablePanelRef.value) {
        rightTablePanelRef.value.expandRow(parentRow.id);
        // 然后选中新添加的子项
        rightTablePanelRef.value.setCurrentRow(newChild.id);
      }
    };

    const handleEditRow = (row: TreeTableData) => {
      // 可以触发编辑模式或显示编辑弹窗
      ElMessage.info("编辑功能，可以双击单元格编辑");
    };

    const handleDeleteRow = (row: TreeTableData) => {
      ElMessageBox.confirm("确定要删除这一行吗？", "警告", {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        type: "warning",
      })
        .then(() => {
          // 递归查找并删除节点
          const deleteNode = (
            data: TreeTableData[],
            targetId: number | string
          ): boolean => {
            for (let i = 0; i < data.length; i++) {
              if (data[i].id === targetId) {
                data.splice(i, 1);
                return true;
              }
              if (data[i].children && deleteNode(data[i].children!, targetId)) {
                return true;
              }
            }
            return false;
          };

          deleteNode(tableTreeData.value, row.id);
          ElMessage.success("删除成功");
        })
        .catch(() => {});
    };

    const handleCopyRow = (row: TreeTableData) => {
      const newItem: TreeTableData = {
        ...row,
        id: Date.now(),
        name: `${row.name} (复制)`,
        children: undefined, // 复制时不包含子项
      };
      tableTreeData.value.push(newItem);
      ElMessage.success("复制成功");
    };

    // 子表格事件处理函数（通用）
    const handleSubTableAddRow = () => {
      ElMessage.info("子表格新增功能");
    };

    const handleSubTableAddChild = () => {
      ElMessage.warning("子表格不支持添加子项");
    };

    const handleSubTableEditRow = (row: any) => {
      ElMessage.info("子表格编辑功能，可以双击单元格编辑");
    };

    const handleSubTableDeleteRow = (row: any) => {
      ElMessage.info("子表格删除功能");
    };

    const handleSubTableCopyRow = (row: any) => {
      ElMessage.info("子表格复制功能");
    };

    // 统计函数
    const getFolderCount = () => {
      const countFolders = (items: TreeTableData[]): number => {
        let count = 0;
        items.forEach((item) => {
          if (item.type === "folder") {
            count++;
          }
          if (item.children && item.children.length > 0) {
            count += countFolders(item.children);
          }
        });
        return count;
      };
      return countFolders(tableTreeData.value);
    };

    const getFileCount = () => {
      const countFiles = (items: TreeTableData[]): number => {
        let count = 0;
        items.forEach((item) => {
          if (item.type === "file") {
            count++;
          }
          if (item.children && item.children.length > 0) {
            count += countFiles(item.children);
          }
        });
        return count;
      };
      return countFiles(tableTreeData.value);
    };

    const getCurrentTime = () => {
      const now = new Date();
      return now.toLocaleString("zh-CN", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    };

    expose({ init });

    return {
      rightTablePanelRef,
      tableTreeData,
      tableColumns,
      activeSubTab,
      historyData,
      detailData,
      statisticsData,
      historyColumns,
      detailColumns,
      statisticsColumns,
      handleSubTabChange,
      handleAddRow,
      handleAddChild,
      handleEditRow,
      handleDeleteRow,
      handleCopyRow,
      handleSubTableAddRow,
      handleSubTableAddChild,
      handleSubTableEditRow,
      handleSubTableDeleteRow,
      handleSubTableCopyRow,
      getFolderCount,
      getFileCount,
      getCurrentTime,
    };
  },
});
</script>

<template>
  <div class="fbfx-view-container">
    <!-- 上半部分：主要表格内容 -->
    <div class="main-content-section">
      <RightTablePanel
        ref="rightTablePanelRef"
        :table-data="tableTreeData"
        :table-columns="tableColumns"
        :is-tree-table="true"
        @add-row="handleAddRow"
        @add-child="handleAddChild"
        @edit-row="handleEditRow"
        @delete-row="handleDeleteRow"
        @copy-row="handleCopyRow"
      />
    </div>

    <!-- 下半部分：子标签页内容 -->
    <div class="sub-tabs-section">
      <el-tabs v-model="activeSubTab" type="card" @tab-change="handleSubTabChange">
        <el-tab-pane label="人材机明细" name="detail">
          <RightTablePanel
            :table-data="detailData"
            :table-columns="detailColumns"
            :is-tree-table="false"
            @add-row="handleSubTableAddRow"
            @add-child="handleSubTableAddChild"
            @edit-row="handleSubTableEditRow"
            @delete-row="handleSubTableDeleteRow"
            @copy-row="handleSubTableCopyRow"
          />
        </el-tab-pane>
        <el-tab-pane label="单价构成" name="statistics">
          <RightTablePanel
            :table-data="statisticsData"
            :table-columns="statisticsColumns"
            :is-tree-table="false"
            @add-row="handleSubTableAddRow"
            @add-child="handleSubTableAddChild"
            @edit-row="handleSubTableEditRow"
            @delete-row="handleSubTableDeleteRow"
            @copy-row="handleSubTableCopyRow"
          />
        </el-tab-pane>
        <el-tab-pane label="标准换算" name="history">
          <RightTablePanel
            :table-data="historyData"
            :table-columns="historyColumns"
            :is-tree-table="false"
            @add-row="handleSubTableAddRow"
            @add-child="handleSubTableAddChild"
            @edit-row="handleSubTableEditRow"
            @delete-row="handleSubTableDeleteRow"
            @copy-row="handleSubTableCopyRow"
          />
        </el-tab-pane>
      </el-tabs>
    </div>

    <!-- 底部统计信息 -->
  </div>
</template>

<style scoped>
.fbfx-view-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 上半部分：主要表格内容 */
.main-content-section {
  height: 55%;
  flex-shrink: 0;
  border-bottom: 1px solid #e4e7ed;
  overflow: hidden;
  margin-bottom: 8px;
}

/* 下半部分：子标签页内容 */
.sub-tabs-section {
  height: calc(45% - 8px);
  flex-shrink: 0;
  background: #fff;
  overflow: hidden;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

/* 底部统计信息 */
.footer-stats {
  height: 32px;
  flex-shrink: 0;
  background: #f5f7fa;
  border-top: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  padding: 0 16px;
}

.stats-container {
  display: flex;
  align-items: center;
  gap: 16px;
  white-space: nowrap;
  overflow: hidden;
}

.stat-item {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.stat-label {
  font-size: 12px;
  color: #606266;
  font-weight: 500;
}

.stat-value {
  font-size: 12px;
  color: #303133;
  font-weight: 600;
}

.stat-value.highlight {
  color: #409eff;
  font-weight: 700;
}

.stat-separator {
  color: #dcdfe6;
  font-size: 12px;
  margin: 0 4px;
}

/* 主要表格内容的 RightTablePanel 样式调整 */
.main-content-section :deep(.table-panel-container) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main-content-section :deep(.el-table) {
  flex: 1;
  height: 100%;
}

.main-content-section :deep(.el-table .el-table__body-wrapper) {
  max-height: calc(100% - 40px);
  overflow-y: auto;
}

.main-content-section :deep(.el-table__header-wrapper) {
  flex-shrink: 0;
}

/* 子标签页中的 RightTablePanel 样式调整 */
.sub-tabs-section :deep(.table-panel-container) {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.sub-tabs-section :deep(.el-table) {
  flex: 1;
  font-size: 12px;
  height: 100%;
}

.sub-tabs-section :deep(.el-table .el-table__body-wrapper) {
  max-height: calc(100% - 40px);
  overflow-y: auto;
  min-height: 200px;
}

.sub-tabs-section :deep(.el-table th) {
  padding: 8px 0;
  font-size: 12px;
}

.sub-tabs-section :deep(.el-table td) {
  padding: 6px 0;
  font-size: 12px;
}

/* 标签页样式调整 */
:deep(.el-tabs__header) {
  margin: 0;
  border-bottom: 1px solid #e4e7ed;
}

:deep(.el-tabs__content) {
  height: calc(100% - 40px);
  padding: 8px;
  overflow: hidden;
}

:deep(.el-tab-pane) {
  height: 100%;
  overflow: hidden;
}

/* 表格样式 */
:deep(.el-table) {
  border: none;
}

:deep(.el-table th) {
  background: #fafafa;
  font-weight: 500;
  font-size: 12px;
}

:deep(.el-table td) {
  font-size: 12px;
}

/* 描述列表样式 */
:deep(.el-descriptions) {
  margin: 0;
}

:deep(.el-descriptions__label) {
  font-weight: 500;
  font-size: 12px;
}

:deep(.el-descriptions__content) {
  font-size: 12px;
}
</style>
