<script lang="ts">
import { defineComponent, ref, reactive } from "vue";
import type { PropType } from "vue";
import type { ElTable } from "element-plus";
import { ElMessage, ElMessageBox } from "element-plus";

// --- Data Interfaces ---
interface TableItem {
  id: number | string;
  [key: string]: any;
}

export interface TableColumn {
  prop: string;
  label: string;
  minWidth?: number;
  width?: number;
  align?: "left" | "center" | "right";
  type?: "date";
}

export default defineComponent({
  name: "RightTablePanel",
  props: {
    tableData: {
      type: Array as PropType<TableItem[]>,
      required: true,
    },
    tableColumns: {
      type: Array as PropType<TableColumn[]>,
      required: true,
    },
    isTreeTable: {
      type: Boolean,
      default: false,
    },
  },
  emits: [
    "update:tableData",
    "add-row",
    "add-child",
    "edit-row",
    "delete-row",
    "copy-row",
  ],
  setup(props, { emit }) {
    const localTableData = ref<TableItem[]>(props.tableData);

    const tableRef = ref<InstanceType<typeof ElTable>>();
    const tableContextMenuVisible = ref(false);
    const tableContextMenuPosition = reactive({ x: 0, y: 0 });
    const currentTableItem = ref<TableItem | null>(null);
    const editingTableCell = ref<{ id: number | string; field: string } | null>(null);

    // 弹窗相关 - 保留用于颜色选择
    // 新增和子项添加逻辑移至父组件

    // 颜色选择相关
    const colorDialogVisible = ref(false);
    const colorValue = ref("#409EFF");

    const handleTableContextMenu = (row: TableItem, column: any, event: MouseEvent) => {
      event.preventDefault();
      currentTableItem.value = row;
      tableContextMenuPosition.x = event.clientX;
      tableContextMenuPosition.y = event.clientY;
      tableContextMenuVisible.value = true;
      document.addEventListener("click", closeTableContextMenu);
    };

    const closeTableContextMenu = () => {
      tableContextMenuVisible.value = false;
      document.removeEventListener("click", closeTableContextMenu);
    };

    const handleTableCommand = (command: string) => {
      if (!currentTableItem.value) return;

      switch (command) {
        case "edit":
          emit("edit-row", currentTableItem.value);
          break;
        case "delete":
          emit("delete-row", currentTableItem.value);
          break;
        case "copy":
          emit("copy-row", currentTableItem.value);
          break;
        case "add-child":
          emit("add-child", currentTableItem.value);
          break;
      }
      tableContextMenuVisible.value = false;
    };

    const handleAddTableRow = () => {
      emit("add-row");
    };

    const handleTableCellDblClick = (row: TableItem, field: string) => {
      editingTableCell.value = { id: row.id, field };
    };
    const finishEditTableCell = () => {
      editingTableCell.value = null;
      emit("update:tableData", localTableData.value);
    };

    // 移除内部的新增逻辑，由父组件处理

    const openColorDialog = () => {
      colorValue.value = currentTableItem.value?.color || "#409EFF";
      colorDialogVisible.value = true;
      tableContextMenuVisible.value = false;
    };

    const handleColorSubmit = () => {
      if (!colorValue.value) {
        ElMessage.warning("请选择颜色");
        return;
      }
      if (currentTableItem.value) {
        currentTableItem.value.color = colorValue.value;
        emit("update:tableData", localTableData.value);
      }
      colorDialogVisible.value = false;
    };

    // 行样式
    const tableRowStyle = ({ row }: { row: any }) => {
      if (row.color && row.color.trim()) {
        return { backgroundColor: `${row.color}` };
      }
      return {};
    };

    // 调试输出，帮助排查表格渲染问题
    console.log("RightTablePanel tableData:", localTableData.value);
    console.log("RightTablePanel tableColumns:", props.tableColumns);

    return {
      localTableData,
      tableRef,
      tableContextMenuVisible,
      tableContextMenuPosition,
      handleTableContextMenu,
      handleTableCommand,
      closeTableContextMenu,
      handleAddTableRow,
      editingTableCell,
      handleTableCellDblClick,
      finishEditTableCell,
      colorDialogVisible,
      colorValue,
      openColorDialog,
      handleColorSubmit,
      tableRowStyle,
    };
  },
});
</script>

<template>
  <div class="table-panel-container">
    <div class="panel-header">
      <el-button type="primary" size="small" @click="handleAddTableRow">
        <el-icon><Plus /></el-icon>
      </el-button>
    </div>
    <!-- 新增和子项添加弹窗移至父组件处理 -->
    <el-dialog v-model="colorDialogVisible" title="选择颜色" width="300px">
      <el-color-picker v-model="colorValue" />
      <template #footer>
        <el-button @click="colorDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleColorSubmit">确定</el-button>
      </template>
    </el-dialog>
    <el-table
      ref="tableRef"
      :data="localTableData"
      style="width: 100%"
      row-key="id"
      border
      @row-contextmenu="handleTableContextMenu"
      :row-style="tableRowStyle"
      :tree-props="isTreeTable ? { children: 'children' } : undefined"
      :default-expand-all="isTreeTable"
    >
      <!-- 展开收起箭头列（仅树形表格，让Element Plus自动处理） -->
      <el-table-column v-if="isTreeTable" width="50" label="" />

      <!-- 普通表格的空白列 -->
      <el-table-column v-if="!isTreeTable" width="50" />

      <el-table-column
        v-for="col in tableColumns"
        :key="col.prop"
        :prop="col.prop"
        :label="col.label"
        :min-width="col.minWidth"
        :width="col.width"
        :align="col.align"
      >
        <template #default="scope">
          <template
            v-if="
              editingTableCell &&
              editingTableCell.id === scope.row.id &&
              editingTableCell.field === col.prop
            "
          >
            <el-input
              v-model="scope.row[col.prop]"
              size="small"
              autofocus
              @blur="finishEditTableCell"
              @keyup.enter="finishEditTableCell"
            />
          </template>
          <template v-else>
            <div v-if="isTreeTable && col.prop === 'name'" class="tree-name-cell">
              <el-icon v-if="scope.row.type === 'folder'" class="folder-icon">
                <Folder />
              </el-icon>
              <el-icon v-else class="file-icon">
                <Document />
              </el-icon>
              <span @dblclick="handleTableCellDblClick(scope.row, col.prop)">
                {{ scope.row[col.prop] }}
              </span>
            </div>
            <div v-else-if="isTreeTable && col.prop === 'type'" class="tree-type-cell">
              <el-tag
                :type="scope.row.type === 'folder' ? 'warning' : 'info'"
                size="small"
              >
                {{ scope.row.type === "folder" ? "文件夹" : "文件" }}
              </el-tag>
            </div>
            <span v-else @dblclick="handleTableCellDblClick(scope.row, col.prop)">
              {{
                scope.row[col.prop] ||
                (col.prop === "size" && scope.row.type === "folder"
                  ? "-"
                  : scope.row[col.prop])
              }}
            </span>
          </template>
        </template>
      </el-table-column>
    </el-table>
    <!-- Table Context Menu -->
    <div
      v-if="tableContextMenuVisible"
      class="custom-context-menu"
      :style="{
        left: tableContextMenuPosition.x + 'px',
        top: tableContextMenuPosition.y + 'px',
        zIndex: 2000,
      }"
    >
      <div class="context-menu-item" @click="handleTableCommand('edit')">
        <el-icon><Edit /></el-icon> 编辑
      </div>
      <div class="context-menu-item" @click="handleTableCommand('delete')">
        <el-icon><Delete /></el-icon> 删除
      </div>
      <div class="context-menu-item" @click="handleTableCommand('copy')">
        <el-icon><CopyDocument /></el-icon> 复制
      </div>
      <div class="context-menu-item" @click="handleTableCommand('add-child')">
        <el-icon><Plus /></el-icon> 添加子项
      </div>
      <div class="context-menu-item" @click="openColorDialog">
        <el-icon><Brush /></el-icon> 添加颜色
      </div>
    </div>
  </div>
</template>

<style scoped>
.table-panel-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.panel-header {
  padding: 16px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  color: #303133;
}
:deep(.el-table .el-input__wrapper) {
  box-shadow: none;
}
:deep(.el-table) {
  --el-table-border-color: #e4e7ed;
  --el-table-header-bg-color: #f5f7fa;
}
:deep(.el-table th) {
  font-weight: 600;
  color: #606266;
}
.custom-context-menu {
  position: fixed;
  min-width: 140px;
  background: #fff;
  border: 1px solid #e4e7ed;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  padding: 4px 0;
  font-size: 14px;
  user-select: none;
}
.context-menu-item {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
}
.context-menu-item:hover {
  background-color: #f5f7fa;
}
:deep(.el-table__row.colored-row) {
  border-left: 4px solid var(--row-color);
}

.tree-name-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tree-type-cell {
  display: flex;
  align-items: center;
}

.folder-icon {
  color: #e6a23c;
  font-size: 16px;
}

.file-icon {
  color: #909399;
  font-size: 16px;
}
</style>
