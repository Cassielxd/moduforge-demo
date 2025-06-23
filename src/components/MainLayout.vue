<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { House, Document, View, Minus, FullScreen, Close, Setting, Clock } from "@element-plus/icons-vue";
import { Window } from '@tauri-apps/api/window';
// @ts-ignore
import SettingsDialog from './SettingsDialog.vue';

const appWindow = new Window('main');
const router = useRouter();
const route = useRoute();

const activeMenu = ref((route.name as string) || "home");
const showSettings = ref(false);

const menuItems = [
  {
    name: "home",
    label: "首页",
    icon: House,
    path: "/"
  },
  {
    name: "history",
    label: "历史记录",
    icon: Clock,
    children: [
      { id: 1, title: '项目 A', timestamp: '2024-03-20 10:00:00', type: '创建' },
      { id: 2, title: '项目 B', timestamp: '2024-03-20 09:30:00', type: '修改' },
      { id: 3, title: '项目 C', timestamp: '2024-03-19 16:45:00', type: '删除' },
    ]
  }
];

const handleMenuSelect = (key: string) => {
  const item = menuItems.find((item) => item.name === key);
  if (item && item.path) {
    router.push(item.path);
    activeMenu.value = key;
  }
};

const minimizeWindow = () => appWindow.minimize();
const maximizeWindow = () => appWindow.toggleMaximize();
const closeWindow = () => appWindow.close();
</script>

<template>
  <div class="main-layout">
    <header class="main-header">
      <div class="header-left">
        <h1 class="app-title">ModuForge Demo</h1>
      </div>
      <div class="header-menu" data-tauri-drag-region>
        <el-menu :default-active="activeMenu" mode="horizontal" @select="handleMenuSelect" background-color="#ffffff"
          text-color="#303133" active-text-color="#409EFF" :ellipsis="false" class="header-nav-menu">
          <template v-for="item in menuItems" :key="item.name">
            <el-sub-menu v-if="item.children" :index="item.name">
              <template #title>
                <el-icon>
                  <component :is="item.icon" />
                </el-icon>
                <span>{{ item.label }}</span>
              </template>
              <div class="history-submenu">
                <div class="history-header">
                  <span>历史记录</span>
                  <el-button type="danger" size="small">清空</el-button>
                </div>
                <el-scrollbar max-height="300px">
                  <div class="history-list">
                    <div v-for="history in item.children" :key="history.id" class="history-item">
                      <div class="history-item-header">
                        <span class="history-title">{{ history.title }}</span>
                        <el-tag size="small"
                          :type="history.type === '创建' ? 'success' : history.type === '修改' ? 'warning' : 'danger'">
                          {{ history.type }}
                        </el-tag>
                      </div>
                      <div class="history-time">{{ history.timestamp }}</div>
                    </div>
                  </div>
                </el-scrollbar>
              </div>
            </el-sub-menu>
            <el-menu-item v-else :index="item.name">
              <el-icon>
                <component :is="item.icon" />
              </el-icon>
              <span>{{ item.label }}</span>
            </el-menu-item>
          </template>
        </el-menu>
      </div>
      <div class="window-controls">
        <button class="window-control-button" @click="showSettings = true">
          <el-icon>
            <Setting />
          </el-icon>
        </button>
        <button class="window-control-button" @click="minimizeWindow">
          <el-icon>
            <Minus />
          </el-icon>
        </button>
        <button class="window-control-button" @click="maximizeWindow">
          <el-icon>
            <FullScreen />
          </el-icon>
        </button>
        <button class="window-control-button close" @click="closeWindow">
          <el-icon>
            <Close />
          </el-icon>
        </button>
      </div>
    </header>
    <div class="main-content">
      <router-view />
    </div>
    <SettingsDialog v-model="showSettings" />
  </div>
</template>

<style scoped>
.main-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  overflow: hidden;
}

.main-header {
  height: 60px;
  display: flex;
  align-items: center;
  padding: 0 20px;
  border-bottom: 1px solid #e4e7ed;
  flex-shrink: 0;
  background-color: #ffffff;
  gap: 40px;
}

.header-left {
  display: flex;
  align-items: center;
}

.app-title {
  font-size: 20px;
  color: #303133;
  margin: 0;
}

.header-menu {
  display: flex;
  width: 100%;
}

.header-nav-menu {
  border-bottom: none !important;
  min-width: auto !important;
}

.header-nav-menu .el-menu-item {
  border-bottom: none !important;
  white-space: nowrap;
  overflow: visible;
  text-overflow: unset;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.window-controls {
  display: flex;
  gap: 8px;
  margin-left: auto;
}

.window-control-button {
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  color: #666;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.window-control-button:hover {
  background-color: #f5f5f5;
}

.window-control-button.close:hover {
  background-color: #ff4d4f;
  color: white;
}

.window-control-button .el-icon {
  font-size: 16px;
}

/* 历史记录子菜单样式 */
.history-submenu {
  min-width: 300px;
  padding: 8px 0;
}

.history-header {
  padding: 8px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #e4e7ed;
  margin-bottom: 8px;
}

.history-header span {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
}

.history-list {
  padding: 0 12px;
}

.history-item {
  padding: 8px 0;
  border-bottom: 1px solid #f0f0f0;
}

.history-item:last-child {
  border-bottom: none;
}

.history-item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.history-title {
  font-size: 13px;
  color: #303133;
  flex: 1;
  margin-right: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-time {
  font-size: 12px;
  color: #909399;
}

:deep(.el-sub-menu__title) {
  height: 60px;
  line-height: 60px;
}

:deep(.el-menu--popup) {
  min-width: 300px;
  padding: 0;
}

:deep(.el-menu--horizontal > .el-sub-menu .el-sub-menu__icon-arrow) {
  right: 0;
}

:deep(.el-menu--horizontal > .el-sub-menu .el-sub-menu__title) {
  padding-right: 20px;
}

:deep(.el-menu--popup-container) {
  right: 0 !important;
  left: auto !important;
}

/* 添加历史记录菜单靠右的样式 */
.header-menu :deep(.el-menu-item:last-child) {
  margin-left: auto;
}
</style>
