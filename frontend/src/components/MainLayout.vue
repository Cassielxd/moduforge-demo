<script lang="ts">
import { defineComponent, ref } from "vue";
import LeftTreePanel from "./LeftTreePanel.vue";
import RightTablePanel, { TableColumn } from "./RightTablePanel.vue";
import {
  Document,
  Folder,
  Setting,
  User,
  Bell,
  RefreshLeft,
  RefreshRight,
} from "@element-plus/icons-vue";

interface Command {
  execute(): void;
  undo(): void;
}

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
    const activeMenu = ref("1");

    const history: Command[] = [];
    const historyIndex = ref(-1);
    const canUndo = ref(false);
    const canRedo = ref(false);

    const updateHistoryState = () => {
      canUndo.value = historyIndex.value >= 0;
      canRedo.value = historyIndex.value < history.length - 1;
    };

    const executeCommand = (command: Command) => {
      // 如果当前不在历史记录末尾，删除当前位置之后的所有记录
      if (historyIndex.value < history.length - 1) {
        history.splice(historyIndex.value + 1);
      }

      command.execute();
      history.push(command);
      historyIndex.value = history.length - 1;
      updateHistoryState();
    };

    const handleUndo = () => {
      if (canUndo.value) {
        const command = history[historyIndex.value];
        command.undo();
        historyIndex.value--;
        updateHistoryState();
      }
    };

    const handleRedo = () => {
      if (canRedo.value) {
        historyIndex.value++;
        const command = history[historyIndex.value];
        command.execute();
        updateHistoryState();
      }
    };

    // 示例：添加一个文件操作命令
    const addFileCommand = (file: any) => {
      const command: Command = {
        execute: () => {
          // 添加文件的具体实现
          console.log("Adding file:", file);
        },
        undo: () => {
          // 撤销添加文件的具体实现
          console.log("Removing file:", file);
        },
      };
      executeCommand(command);
    };

    const handleNodeSelected = (node: any) => {
      console.log("Node selected in main layout:", node);
      // Here you can add logic to filter the table based on the selected node
    };

    const handleCommand = (command: string) => {
      console.log("Menu command:", command);
      // 处理菜单命令
    };

    const handleMenuSelect = (index: string) => {
      console.log("Menu selected:", index);
      // 处理菜单选择
      switch (index) {
        case "1-1":
          // 处理所有文件
          break;
        case "1-2":
          // 处理最近文件
          break;
        case "1-3":
          // 处理已共享
          break;
        case "1-4":
          // 处理回收站
          break;
        case "2-1":
          // 处理我的项目
          break;
        case "2-2":
          // 处理团队项目
          break;
        case "2-3":
          // 处理项目模板
          break;
        case "3-1":
          // 处理个人设置
          break;
        case "3-2":
          // 处理团队设置
          break;
        case "3-3":
          // 处理系统设置
          break;
      }
    };

    return {
      treeData,
      panes,
      activeTab,
      activeMenu,
      handleNodeSelected,
      handleCommand,
      handleMenuSelect,
      handleUndo,
      handleRedo,
      canUndo,
      canRedo,
      Document,
      Folder,
      Setting,
      User,
      Bell,
      RefreshLeft,
      RefreshRight,
    };
  },
});
</script>

<template>
  <div class="main-layout">
    <header class="main-header">
      <div class="header-left">
        <h1 class="app-title">ModuForge Demo</h1>
        <div class="history-buttons">
          <el-tooltip content="回退" placement="bottom">
            <el-button :disabled="!canUndo" circle @click="handleUndo">
              <el-icon><RefreshLeft /></el-icon>
            </el-button>
          </el-tooltip>
          <el-tooltip content="重做" placement="bottom">
            <el-button :disabled="!canRedo" circle @click="handleRedo">
              <el-icon><RefreshRight /></el-icon>
            </el-button>
          </el-tooltip>
        </div>
        <el-menu
          mode="horizontal"
          :ellipsis="false"
          v-model="activeMenu"
          class="main-menu"
          @select="handleMenuSelect"
        >
          <el-sub-menu index="1">
            <template #title>
              <el-icon><Document /></el-icon>
              <span>文件</span>
            </template>
            <el-menu-item index="1-1">所有文件</el-menu-item>
            <el-menu-item index="1-2">最近文件</el-menu-item>
            <el-menu-item index="1-3">已共享</el-menu-item>
            <el-menu-item index="1-4">回收站</el-menu-item>
          </el-sub-menu>

          <el-sub-menu index="2">
            <template #title>
              <el-icon><Folder /></el-icon>
              <span>项目</span>
            </template>
            <el-menu-item index="2-1">我的项目</el-menu-item>
            <el-menu-item index="2-2">团队项目</el-menu-item>
            <el-menu-item index="2-3">项目模板</el-menu-item>
          </el-sub-menu>

          <el-sub-menu index="3">
            <template #title>
              <el-icon><Setting /></el-icon>
              <span>设置</span>
            </template>
            <el-menu-item index="3-1">个人设置</el-menu-item>
            <el-menu-item index="3-2">团队设置</el-menu-item>
            <el-menu-item index="3-3">系统设置</el-menu-item>
          </el-sub-menu>
        </el-menu>
      </div>
      <div class="header-right">
        <el-dropdown @command="handleCommand">
          <el-button type="primary" plain>
            <el-icon><User /></el-icon>
            用户
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="profile">个人信息</el-dropdown-item>
              <el-dropdown-item command="settings">账号设置</el-dropdown-item>
              <el-dropdown-item divided command="logout">退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        <el-badge :value="3" class="notification-badge">
          <el-button circle>
            <el-icon><Bell /></el-icon>
          </el-button>
        </el-badge>
      </div>
    </header>
    <div class="main-content">
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
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue",
    Arial, sans-serif;
  background-color: #f5f7fa;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

.main-header {
  height: 60px;
  background-color: #fff;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.app-title {
  margin: 0;
  font-size: 20px;
  color: #303133;
  font-weight: 600;
}

.history-buttons {
  display: flex;
  gap: 8px;
  margin-right: 8px;
}

.history-buttons .el-button {
  padding: 8px;
}

.history-buttons .el-icon {
  font-size: 16px;
}

.main-menu {
  border-bottom: none;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.notification-badge {
  margin-left: 8px;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  height: calc(100vh - 60px);
}

.left-panel {
  width: 280px;
  min-width: 200px;
  background-color: #fff;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
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
  flex: 1;
  background-color: #fff;
  display: flex;
  flex-direction: column;
  padding: 16px;
  height: 100%;
  overflow: auto;
}

:deep(.el-menu--horizontal) {
  border-bottom: none;
}

:deep(.el-menu-item),
:deep(.el-sub-menu__title) {
  display: flex;
  align-items: center;
  gap: 4px;
}

:deep(.el-menu-item .el-icon),
:deep(.el-sub-menu__title .el-icon) {
  margin-right: 4px;
}

:deep(.el-sub-menu .el-sub-menu__title) {
  padding: 0 20px;
}

:deep(.el-menu--popup) {
  min-width: 120px;
}

:deep(.el-menu--popup .el-menu-item) {
  padding: 8px 20px;
}
</style>
