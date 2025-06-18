<script lang="ts">
import { defineComponent, ref } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import RightTablePanel, { TableColumn } from "../components/RightTablePanel.vue";

interface TableData {
  id: number | string;
  date: string;
  level: string;
  message: string;
}

export default defineComponent({
  name: "CxxmView",
  components: { RightTablePanel },
  setup(props, { expose }) {
    const tableData = ref<TableData[]>([
      { id: 1, date: "2024-07-29", level: "info", message: "用户登录" },
      { id: 2, date: "2024-07-29", level: "warn", message: "密码即将过期" },
    ]);

    const tableColumns: TableColumn[] = [
      { prop: "date", label: "时间", minWidth: 150 },
      { prop: "level", label: "级别", minWidth: 80 },
      { prop: "message", label: "消息", minWidth: 300 },
    ];

    const rightTablePanelRef = ref();

    const init = (id: string | number | null) => {
      console.log("LogView.vue init called with ID:", id);
      // 如果需要，这里也可以根据ID刷新日志数据
      if (id) {
        tableData.value = [
          {
            id: id,
            date: new Date().toISOString(),
            level: "info",
            message: `Logs related to tree node ${id}`,
          },
        ];
      }
    };

    // 处理表格事件
    const handleAddRow = (currentRow?: TableData) => {
      const newRow: TableData = {
        id: Date.now(),
        date: new Date().toISOString().split("T")[0],
        level: "info",
        message: "新日志记录",
      };

      if (currentRow) {
        // 在指定行的下一行插入新行
        const currentIndex = tableData.value.findIndex((item) => item.id === currentRow.id);
        if (currentIndex !== -1) {
          tableData.value.splice(currentIndex + 1, 0, newRow);
          ElMessage.success("在选中行下方添加成功");
        } else {
          tableData.value.push(newRow);
          ElMessage.success("添加成功");
        }
      } else {
        tableData.value.push(newRow);
        ElMessage.success("添加成功");
      }

      // 选中新添加的行
      if (rightTablePanelRef.value) {
        rightTablePanelRef.value.setCurrentRow(newRow.id);
      }
    };

    const handleAddChild = (parentRow: TableData) => {
      // 普通表格不支持子项，显示提示
      ElMessage.warning("此表格不支持添加子项");
    };

    const handleEditRow = (row: TableData) => {
      // 可以触发编辑模式或显示编辑弹窗
      ElMessage.info("编辑功能，可以双击单元格编辑");
    };

    const handleDeleteRow = (row: TableData) => {
      ElMessageBox.confirm("确定要删除这一行吗？", "警告", {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        type: "warning",
      })
        .then(() => {
          const index = tableData.value.findIndex((item) => item.id === row.id);
          if (index > -1) {
            tableData.value.splice(index, 1);
            ElMessage.success("删除成功");
          }
        })
        .catch(() => {});
    };

    const handleCopyRow = (row: TableData) => {
      const newItem: TableData = {
        ...row,
        id: Date.now(),
        message: `${row.message} (复制)`,
      };
      tableData.value.push(newItem);
      ElMessage.success("复制成功");
    };

    expose({ init });

    return {
      rightTablePanelRef,
      tableData,
      tableColumns,
      handleAddRow,
      handleAddChild,
      handleEditRow,
      handleDeleteRow,
      handleCopyRow,
    };
  },
});
</script>

<template>
  <RightTablePanel 
    ref="rightTablePanelRef"
    :table-data="tableData" 
    :table-columns="tableColumns" 
    @add-row="handleAddRow"
    @add-child="handleAddChild"
    @edit-row="handleEditRow"
    @delete-row="handleDeleteRow"
    @copy-row="handleCopyRow"
  />
</template>
