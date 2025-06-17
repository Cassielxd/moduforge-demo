<script lang="ts">
import { defineComponent, ref, reactive } from "vue";
import type { PropType } from "vue";
import type { ElTable } from "element-plus";
import { ElMessage, ElMessageBox } from "element-plus";

// --- Data Interfaces ---
interface TableItem {
  id: number;
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
  },
  emits: ["update:tableData"],
  setup(props, { emit }) {
    const localTableData = ref<TableItem[]>(props.tableData);

    const tableRef = ref<InstanceType<typeof ElTable>>();
    const tableContextMenuVisible = ref(false);
    const tableContextMenuPosition = reactive({ x: 0, y: 0 });
    const currentTableItem = ref<TableItem | null>(null);
    const editingTableCell = ref<{ id: number; field: string } | null>(null);

    // 新增弹窗相关
    const addDialogVisible = ref(false);
    const addForm = ref({ name: "", date: "", address: "" });

    // 添加子项弹窗相关
    const addChildDialogVisible = ref(false);
    const addChildForm = ref({ name: "", date: "", address: "" });

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
          // Placeholder for edit logic if needed from context menu
          break;
        case "delete":
          ElMessageBox.confirm("确定要删除这一行吗？", "警告", {
            confirmButtonText: "确定",
            cancelButtonText: "取消",
            type: "warning",
          })
            .then(() => {
              const index = localTableData.value.findIndex(
                (item) => item.id === currentTableItem.value?.id
              );
              localTableData.value.splice(index, 1);
              ElMessage.success("删除成功");
              emit("update:tableData", localTableData.value);
            })
            .catch(() => {});
          break;
        case "copy":
          const newItem = {
            ...currentTableItem.value,
            id: Date.now(),
            name: `${currentTableItem.value.name} (复制)`,
          };
          localTableData.value.push(newItem);
          ElMessage.success("复制成功");
          emit("update:tableData", localTableData.value);
          break;
      }
      tableContextMenuVisible.value = false;
    };

    const handleAddTableRow = () => {
      const newRow: TableItem = {
        id: Date.now(),
        date: new Date().toISOString().split("T")[0],
        name: "新文件",
        address: "请输入描述",
      };
      localTableData.value.push(newRow);
      ElMessage.success("添加成功");
      emit("update:tableData", localTableData.value);
    };

    const handleTableCellDblClick = (row: TableItem, field: string) => {
      editingTableCell.value = { id: row.id, field };
    };
    const finishEditTableCell = () => {
      editingTableCell.value = null;
      emit("update:tableData", localTableData.value);
    };

    const openAddDialog = () => {
      addForm.value = { name: "", date: "", address: "" };
      addDialogVisible.value = true;
    };

    const handleAddSubmit = () => {
      const newRow: TableItem = {
        id: Date.now(),
        ...addForm.value,
      };
      localTableData.value.push(newRow);
      ElMessage.success("添加成功");
      emit("update:tableData", localTableData.value);
      addDialogVisible.value = false;
    };

    const openAddChildDialog = () => {
      addChildForm.value = { name: "", date: "", address: "" };
      addChildDialogVisible.value = true;
      tableContextMenuVisible.value = false;
    };

    const handleAddChildSubmit = () => {
      const newChild = {
        id: Date.now(),
        ...addChildForm.value,
      };
      if (currentTableItem.value && !currentTableItem.value.children) {
        currentTableItem.value.children = [];
      }
      if (currentTableItem.value) {
        currentTableItem.value.children.push(newChild);
        ElMessage.success("添加子项成功");
        emit("update:tableData", localTableData.value);
        // 自动展开父节点
        if (tableRef.value) {
          tableRef.value.toggleRowExpansion(currentTableItem.value, true);
        }
      }
      addChildDialogVisible.value = false;
    };

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
      addDialogVisible,
      addForm,
      openAddDialog,
      handleAddSubmit,
      addChildDialogVisible,
      addChildForm,
      openAddChildDialog,
      handleAddChildSubmit,
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
      <el-button type="primary" size="small" @click="openAddDialog">
        <el-icon><Plus /></el-icon>
      </el-button>
    </div>
    <el-dialog v-model="addDialogVisible" title="新增行" width="400px">
      <el-form :model="addForm">
        <el-form-item label="名称"><el-input v-model="addForm.name" /></el-form-item>
        <el-form-item label="日期"><el-input v-model="addForm.date" /></el-form-item>
        <el-form-item label="描述"><el-input v-model="addForm.address" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleAddSubmit">确定</el-button>
      </template>
    </el-dialog>
    <el-dialog v-model="addChildDialogVisible" title="添加子项" width="400px">
      <el-form :model="addChildForm">
        <el-form-item label="名称"><el-input v-model="addChildForm.name" /></el-form-item>
        <el-form-item label="日期"><el-input v-model="addChildForm.date" /></el-form-item>
        <el-form-item label="描述"
          ><el-input v-model="addChildForm.address"
        /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addChildDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleAddChildSubmit">确定</el-button>
      </template>
    </el-dialog>
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
    >
      <el-table-column width="50" />
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
            <span @dblclick="handleTableCellDblClick(scope.row, col.prop)">{{
              scope.row[col.prop]
            }}</span>
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
      <div class="context-menu-item" @click="openAddChildDialog">
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
</style>
