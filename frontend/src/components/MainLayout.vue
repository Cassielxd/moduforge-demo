<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { House, Document, View } from "@element-plus/icons-vue";

const router = useRouter();
const route = useRoute();

const activeMenu = ref((route.name as string) || "home");

const menuItems = [{ name: "home", label: "首页", icon: House, path: "/" }];

const handleMenuSelect = (key: string) => {
  const item = menuItems.find((item) => item.name === key);
  if (item) {
    router.push(item.path);
    activeMenu.value = key;
  }
};
</script>

<template>
  <div class="main-layout">
    <header class="main-header">
      <div class="header-left">
        <h1 class="app-title">ModuForge Demo</h1>
      </div>
      <div class="header-menu">
        <el-menu
          :default-active="activeMenu"
          mode="horizontal"
          @select="handleMenuSelect"
          background-color="#ffffff"
          text-color="#303133"
          active-text-color="#409EFF"
          :ellipsis="false"
          class="header-nav-menu"
        >
          <el-menu-item v-for="item in menuItems" :key="item.name" :index="item.name">
            <el-icon><component :is="item.icon" /></el-icon>
            <span>{{ item.label }}</span>
          </el-menu-item>
        </el-menu>
      </div>
    </header>
    <div class="main-content">
      <router-view />
    </div>
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
</style>
