<script lang="ts">
import { defineComponent, ref } from "vue";
import LeftTreePanel from "./LeftTreePanel.vue";
import RightTablePanel, { TableColumn } from "./RightTablePanel.vue";

export default defineComponent({
  name: "MainLayout",
  components: { LeftTreePanel, RightTablePanel },
  setup() {
    const treeData = ref([
      {
        id: 1,
        label: "Documents",
        children: [
          { id: 4, label: "Projects" },
          { id: 5, label: "Photos" },
        ],
      },
      {
        id: 2,
        label: "Downloads",
        children: [
          { id: 6, label: "Apps" },
          { id: 7, label: "Music" },
        ],
      },
      { id: 3, label: "Desktop" },
    ]);

    // 多表格数据
    const panes = ref([
      {
        name: "files",
        label: "文件列表",
        tableData: [
          {
            id: 1,
            date: "2024-07-26",
            name: "Folder 1",
            address: "Some info about Folder 1",
            children: [
              {
                id: 3,
                date: "2024-07-27",
                name: "File 1-1.txt",
                address: "Some info about File 1-1",
              },
            ],
          },
          {
            id: 2,
            date: "2024-07-28",
            name: "File 2.png",
            address: "Some info about File 2",
          },
        ],
        tableColumns: [
          { prop: "name", label: "名称", minWidth: 150 },
          {
            prop: "date",
            label: "日期",
            minWidth: 120,
            align: "center" as "center",
            type: "date" as "date",
          },
          { prop: "address", label: "描述", minWidth: 200 },
        ] as TableColumn[],
      },
      {
        name: "logs",
        label: "日志",
        tableData: [
          { id: 1, time: "2024-07-26 10:00", level: "info", message: "系统启动" },
          { id: 2, time: "2024-07-26 10:05", level: "warn", message: "磁盘空间不足" },
        ],
        tableColumns: [
          { prop: "time", label: "时间", minWidth: 160 },
          { prop: "level", label: "级别", minWidth: 80 },
          { prop: "message", label: "内容", minWidth: 200 },
        ] as TableColumn[],
      },
    ]);
    const activeTab = ref("files");

    const handleNodeSelected = (node: any) => {
      console.log("Node selected in main layout:", node);
      // Here you can add logic to filter the table based on the selected node
    };

    return {
      treeData,
      panes,
      activeTab,
      handleNodeSelected,
    };
  },
});
</script>

<template>
  <div class="main-layout">
    <div class="left-panel">
      <LeftTreePanel :tree-data="treeData" @node-selected="handleNodeSelected" />
    </div>
    <div class="resizer"></div>
    <div class="right-panel">
      <el-tabs v-model="activeTab">
        <el-tab-pane
          v-for="pane in panes"
          :key="pane.name"
          :label="pane.label"
          :name="pane.name"
        >
          <RightTablePanel
            :table-data="pane.tableData"
            :table-columns="pane.tableColumns"
          />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue",
    Arial, sans-serif;
  background-color: #f5f7fa;
}

.left-panel {
  width: 280px;
  min-width: 200px;
  background-color: #fff;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
}

.resizer {
  width: 5px;
  cursor: col-resize;
  background-color: #e4e7ed;
  z-index: 10;
}

.resizer:hover {
  background-color: #409eff;
}

.right-panel {
  flex-grow: 1;
  background-color: #fff;
  display: flex;
  flex-direction: column;
  padding: 16px;
}
</style>
