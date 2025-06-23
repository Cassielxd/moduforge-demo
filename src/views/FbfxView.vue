<script lang="ts">
import { defineComponent, ref, nextTick, reactive, onMounted, onUnmounted, watch } from "vue";
import { ElMessage, ElMessageBox, ElDialog, ElButton, ElColorPicker, ElIcon, ElTabs, ElTabPane } from "element-plus";
// @ts-ignore
import { TabulatorFull as Tabulator } from 'tabulator-tables';
// @ts-ignore
import type { RowComponent, CellComponent } from 'tabulator-tables';
import 'tabulator-tables/dist/css/tabulator.min.css';
import { Plus, Edit, Delete, CopyDocument, Brush, Folder, Document } from '@element-plus/icons-vue';
import { useMainTabulator } from '@/composables/useMainTabulator';
import { useSubTabulator } from '@/composables/useSubTabulator';

interface TreeTableData {
  id: number | string;
  name: string;
  type: string;
  size?: string;
  date: string;
  children?: TreeTableData[];
  color?: string;
}

interface TableColumn {
  prop: string;
  label: string;
  minWidth?: number;
  width?: number;
  align?: "left" | "center" | "right";
  type?: "date";
}

// 将主表格数据转换为useMainTabulator所需的格式
const convertToMainTabulatorFormat = (data: TreeTableData[]): any[] => {
  return data.map(item => ({
    id: String(item.id),
    name: item.name,
    type: item.type,
    subType: item.size || '',
    description: item.date,
    children: item.children ? convertToMainTabulatorFormat(item.children) : undefined,
    _row_color: item.color
  }));
};

export default defineComponent({
  name: "FbfxView",
  components: {
    ElDialog,
    ElButton,
    ElColorPicker,
    ElIcon,
    ElTabs,
    ElTabPane,
    Plus,
    Edit,
    Delete,
    CopyDocument,
    Brush,
    Folder,
    Document,
  },
  setup(props, { expose }) {
    // 使用主表格组合式函数
    const mainTabulatorComposable = useMainTabulator();
    // 使用子表格组合式函数
    const subTabulatorComposable = useSubTabulator();

    // 主表格数据
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

    // 子标签页状态
    const activeSubTab = ref("detail");

    // ========== Tabulator 相关变量 ==========
    const mainTableRef = ref<HTMLElement>();
    const subTableRef = ref<HTMLElement>();

    // 子表格数据
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
    ]);

    const detailData = ref([
      { id: 1, property: "文件类型", value: "目录/文件" },
      { id: 2, property: "创建时间", value: "2024-07-29" },
      { id: 3, property: "修改时间", value: "2024-07-29" },
      { id: 4, property: "文件大小", value: "2.5MB" },
    ]);

    const statisticsData = ref([
      {
        id: 1,
        name: "总文件数",
        value: "156",
        unit: "个",
        description: "包含所有文件和文件夹",
      },
      {
        id: 2,
        name: "文件夹数",
        value: "23",
        unit: "个",
        description: "目录文件夹数量",
      },
    ]);

    // 子表格列配置
    const historyColumns = [
      { prop: "time", label: "时间", width: 150 },
      { prop: "action", label: "操作", width: 80 },
      { prop: "file", label: "文件", minWidth: 200 },
      { prop: "user", label: "用户", width: 100 },
    ];

    const detailColumns = [
      { prop: "property", label: "属性", width: 120 },
      { prop: "value", label: "值", minWidth: 200 },
    ];

    const statisticsColumns = [
      { prop: "name", label: "统计项", width: 120 },
      { prop: "value", label: "数值", width: 80, align: "right" as const },
      { prop: "unit", label: "单位", width: 60 },
      { prop: "description", label: "说明", minWidth: 200 },
    ];

    const currentTableItem = ref<TreeTableData | null>(null);
    const currentRowKey = ref<string | number | null>(null);

    // 颜色选择相关
    const colorDialogVisible = ref(false);
    const colorValue = ref("#409EFF");

    // 颜色变化处理函数
    const handleColorChange = (id: string, color: string) => {
      // 在原始数据中更新颜色
      const updateColor = (items: TreeTableData[]): void => {
        for (const item of items) {
          if (String(item.id) === id) {
            item.color = color;
            return;
          }
          if (item.children) {
            updateColor(item.children);
          }
        }
      };

      updateColor(tableTreeData.value);
      ElMessage.success(`已设置颜色：${color}`);
    };

    // 行点击处理函数
    const handleRowClick = (data: any) => {
      currentRowKey.value = data.id;
      console.log('选中行:', data);
    };



    // ========== 主表格事件处理 ==========
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

      // 数据会通过watch自动更新到表格

      // 选中新添加的行
      nextTick(() => {
        setCurrentRow(newRow.id);
      });
    };

    const handleAddChild = (parentRow: TreeTableData, row: RowComponent) => {
      const newChild: TreeTableData = {
        id: Date.now(),
        name: "新子项",
        type: "file",
        size: "0KB",
        date: new Date().toISOString().split("T")[0],
      };

      console.log('添加子项到父行:', parentRow);

      if (!parentRow.children) {
        parentRow.children = [];
      }
      parentRow.children.push(newChild);

      ElMessage.success("添加子项成功");

      // 数据会通过watch自动更新到表格，展开状态会自动保持
    };

    const handleEditRow = (row: TreeTableData) => {
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

          // 数据会通过watch自动更新到表格
          ElMessage.success("删除成功");
        })
        .catch(() => { });
    };

    const handleCopyRow = (row: TreeTableData) => {
      const newItem: TreeTableData = {
        ...row,
        id: Date.now(),
        name: `${row.name} (复制)`,
        children: undefined, // 复制时不包含子项
      };
      tableTreeData.value.push(newItem);

      // 数据会通过watch自动更新到表格

      ElMessage.success("复制成功");

      // 选中复制的新行
      nextTick(() => {
        setCurrentRow(newItem.id);
      });
    };

    // ========== 子表格事件处理 ==========
    const handleSubTableAddRow = () => {
      ElMessage.info("子表格新增功能");
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

    // ========== 颜色选择相关 ==========
    const openColorDialog = () => {
      colorValue.value = currentTableItem.value?.color || "#409EFF";
      colorDialogVisible.value = true;
    };

    const handleColorSubmit = () => {
      if (!colorValue.value) {
        ElMessage.warning("请选择颜色");
        return;
      }
      if (currentTableItem.value) {
        // 使用组合式函数的颜色更新功能
        handleColorChange(String(currentTableItem.value.id), colorValue.value);
      }
      colorDialogVisible.value = false;
    };

    // ========== 工具函数 ==========
    const setCurrentRow = (rowId: string | number) => {
      currentRowKey.value = rowId;
      // 使用组合式函数选择行
      mainTabulatorComposable.selectRow(String(rowId));
    };

    // 初始化子表格
    const initSubTabulator = () => {
      if (!subTableRef.value) return;

      let data, columns;
      switch (activeSubTab.value) {
        case "detail":
          data = detailData.value;
          columns = detailColumns;
          break;
        case "statistics":
          data = statisticsData.value;
          columns = statisticsColumns;
          break;
        case "history":
          data = historyData.value;
          columns = historyColumns;
          break;
        default:
          data = detailData.value;
          columns = detailColumns;
      }

      // 使用子表格组合式函数初始化
      subTabulatorComposable.initSubTabulator(
        subTableRef.value,
        activeSubTab.value,
        data,
        columns
      );

      // 设置子表格事件处理器
      subTabulatorComposable.setEventHandlers({
        onAddRow: handleSubTableAddRow,
        onEditRow: handleSubTableEditRow,
        onDeleteRow: handleSubTableDeleteRow,
        onCopyRow: handleSubTableCopyRow,
      });
    };

    // 处理子标签页切换
    const handleSubTabChange = (tabName: string | number) => {
      console.log("FbfxView: Sub tab changed to:", tabName);
      activeSubTab.value = tabName as string;

      // 重新初始化子表格
      nextTick(() => {
        initSubTabulator();
      });
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

    const init = (id: string | number | null) => {
      console.log("FbfxView.vue init called with ID:", id);
    };

    // ========== 生命周期 ==========
    onMounted(() => {
      nextTick(() => {
        if (mainTableRef.value) {
          const convertedData = convertToMainTabulatorFormat(tableTreeData.value);
          mainTabulatorComposable.initMainTabulator(
            mainTableRef.value,
            convertedData,
            handleColorChange,
            handleRowClick
          );
        }
        initSubTabulator();
      });
    });

    onUnmounted(() => {
      mainTabulatorComposable.destroy();
      subTabulatorComposable.destroy();
    });

    // 监听数据变化
    watch(() => tableTreeData.value, (newData) => {
      const convertedData = convertToMainTabulatorFormat(newData);
      mainTabulatorComposable.updateData(convertedData);
    }, { deep: true });

    expose({ init });

    return {
      mainTableRef,
      subTableRef,
      tableTreeData,
      tableColumns,
      activeSubTab,
      historyData,
      detailData,
      statisticsData,
      historyColumns,
      detailColumns,
      statisticsColumns,
      colorDialogVisible,
      colorValue,
      handleSubTabChange,
      handleAddRow,
      handleAddChild,
      handleEditRow,
      handleDeleteRow,
      handleCopyRow,
      handleSubTableAddRow,
      handleSubTableEditRow,
      handleSubTableDeleteRow,
      handleSubTableCopyRow,
      handleColorSubmit,
      getFolderCount,
      getFileCount,
      getCurrentTime,
      // 暴露主表格组合式函数的所有方法
      ...mainTabulatorComposable,
    };
  },
});
</script>

<template>
  <div class="fbfx-view-container">
    <!-- 颜色选择对话框 -->
    <el-dialog v-model="colorDialogVisible" title="设置边框颜色" width="300px">
      <el-color-picker v-model="colorValue" />
      <template #footer>
        <el-button @click="colorDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleColorSubmit">确定</el-button>
      </template>
    </el-dialog>

    <!-- 上半部分：主要表格内容 -->
    <div class="main-content-section">
      <div ref="mainTableRef" class="main-table-container"></div>
    </div>

    <!-- 下半部分：子标签页内容 -->
    <div class="sub-tabs-section">
      <el-tabs v-model="activeSubTab" type="card" @tab-change="handleSubTabChange">
        <el-tab-pane label="人材机明细" name="detail">
          <div v-if="activeSubTab === 'detail'" ref="subTableRef" class="sub-table-container"></div>
        </el-tab-pane>
        <el-tab-pane label="单价构成" name="statistics">
          <div v-if="activeSubTab === 'statistics'" ref="subTableRef" class="sub-table-container"></div>
        </el-tab-pane>
        <el-tab-pane label="标准换算" name="history">
          <div v-if="activeSubTab === 'history'" ref="subTableRef" class="sub-table-container"></div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <!-- 底部统计信息 -->
    <div class="footer-stats">
      <div class="stats-container">
        <div class="stat-item">
          <span class="stat-label">总文件数</span>
          <span class="stat-value">{{ getFolderCount() + getFileCount() }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">文件夹数</span>
          <span class="stat-value">{{ getFolderCount() }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">总大小</span>
          <span class="stat-value">{{ (getFolderCount() + getFileCount()) * 0.001 }} MB</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">最近修改</span>
          <span class="stat-value">{{ getCurrentTime() }}</span>
        </div>
      </div>
    </div>
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

/* 表格容器样式 */
.main-table-container,
.sub-table-container {
  width: 100%;
  height: 100%;
}

/* Tabulator 自定义样式 */
:deep(.tabulator) {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  background: #fff;
}

:deep(.tabulator .tabulator-header) {
  background: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
}

:deep(.tabulator .tabulator-header .tabulator-col) {
  background: #f5f7fa;
  border-right: 1px solid #e4e7ed;
  font-weight: 600;
  color: #606266;
}

:deep(.tabulator .tabulator-row) {
  border-bottom: 1px solid #e4e7ed;
  position: relative;
}

:deep(.tabulator .tabulator-row .tabulator-cell) {
  border-right: 1px solid #e4e7ed;
  padding: 8px 12px;
}

/* 有颜色边框的行，调整单元格边框 */
:deep(.tabulator .tabulator-row.colored-border .tabulator-cell) {
  border-right: none !important;
}

:deep(.tabulator .tabulator-row:hover) {
  background-color: #f5f7fa;
}

:deep(.tabulator .tabulator-row.tabulator-selected) {
  background-color: #ecf5ff;
}

/* 自定义行选中样式 */
:deep(.tabulator .tabulator-row.row-selected) {
  background-color: #e6f7ff !important;
  border-left: 1px solid #1890ff !important;
  position: relative;
}

/* 选中行悬停效果 */
:deep(.tabulator .tabulator-row.row-selected:hover) {
  background-color: #bae7ff !important;
}

/* 选中行的单元格样式 */
:deep(.tabulator .tabulator-row.row-selected .tabulator-cell) {
  border-color: #91d5ff;
  font-weight: 500;
}

/* 确保有颜色边框的选中行样式正确叠加 */
:deep(.tabulator .tabulator-row.row-selected.colored-border) {
  box-shadow: inset 0 0 0 2px #1890ff, 0 0 0 2px currentColor;
}

/* 树形表格样式 */
.tree-name-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.folder-icon {
  color: #e6a23c;
  font-size: 16px;
}

.file-icon {
  color: #909399;
  font-size: 16px;
}

.type-tag {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.folder-tag {
  background: #fdf6ec;
  color: #e6a23c;
  border: 1px solid #f5dab1;
}

.file-tag {
  background: #f4f4f5;
  color: #909399;
  border: 1px solid #d3d4d6;
}

/* 树形控制列样式 */
:deep(.tabulator .tabulator-cell[data-field="tree_control"]) {
  padding: 4px 8px;
  text-align: center;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control) {
  display: inline-block;
  width: 16px;
  height: 16px;
  line-height: 16px;
  text-align: center;
  cursor: pointer;
  border-radius: 2px;
  transition: background-color 0.2s;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control:hover) {
  background-color: #f0f0f0;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control .tabulator-tree-control-expand) {
  color: #409eff;
  font-weight: bold;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control .tabulator-tree-control-collapse) {
  color: #409eff;
  font-weight: bold;
}

/* 右键菜单样式覆盖 */
:deep(.tabulator-menu) {
  background: #fff;
  border: 1px solid #e4e7ed;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  padding: 4px 0;
  font-size: 14px;
}

:deep(.tabulator-menu .tabulator-menu-item) {
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
}

:deep(.tabulator-menu .tabulator-menu-item:hover) {
  background-color: #f5f7fa;
}

/* 有边框颜色的行样式 */
:deep(.tabulator .tabulator-row.colored-border) {
  border-width: 2px !important;
  border-style: solid !important;
  margin: 1px 0 !important;
  /* 防止边框重叠 */
}

/* 确保边框颜色在悬停时也保持 */
:deep(.tabulator .tabulator-row.colored-border:hover) {
  border-width: 2px !important;
  border-style: solid !important;
}
</style>
