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

    const init = (id: string | number | null) => {
      console.log("FbfxView.vue init called with ID:", id);
      // 根据ID可以刷新不同的文件数据
    };

    // 处理表格事件
    const handleAddRow = () => {
      const newRow: TreeTableData = {
        id: Date.now(),
        name: "新文件",
        type: "file",
        size: "0KB",
        date: new Date().toISOString().split("T")[0],
      };
      tableTreeData.value.push(newRow);
      ElMessage.success("添加成功");
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

    expose({ init });

    return {
      tableTreeData,
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
    :table-data="tableTreeData"
    :table-columns="tableColumns"
    :is-tree-table="true"
    @add-row="handleAddRow"
    @add-child="handleAddChild"
    @edit-row="handleEditRow"
    @delete-row="handleDeleteRow"
    @copy-row="handleCopyRow"
  />
</template>
