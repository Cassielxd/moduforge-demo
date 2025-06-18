<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { House, Document, View, Minus, FullScreen, Close, Setting } from "@element-plus/icons-vue";
import { Window } from '@tauri-apps/api/window';
// @ts-ignore
import SettingsDialog from './SettingsDialog.vue';

const appWindow = new Window('main');
const router = useRouter();
const route = useRoute();

const activeMenu = ref((route.name as string) || "home");
const showSettings = ref(false);

const menuItems = [{ name: "home", label: "首页", icon: House, path: "/" }];

const handleMenuSelect = (key: string) => {
  const item = menuItems.find((item) => item.name === key);
  if (item) {
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
    <header class="main-header" data-tauri-drag-region>
      <div class="header-left">
        <h1 class="app-title">ModuForge Demo</h1>
      </div>
      <div class="header-menu">
        <el-menu :default-active="activeMenu" mode="horizontal" @select="handleMenuSelect" background-color="#ffffff"
          text-color="#303133" active-text-color="#409EFF" :ellipsis="false" class="header-nav-menu">
          <el-menu-item v-for="item in menuItems" :key="item.name" :index="item.name">
            <el-icon>
              <component :is="item.icon" />
            </el-icon>
            <span>{{ item.label }}</span>
          </el-menu-item>
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
</style>
