<script setup lang="ts">
import { ref, toRaw } from "vue";
import LeftTreePanel from "../components/LeftTreePanel.vue";
// @ts-ignore
import MultiTabView from "../components/MultiTabView.vue";
import type { ComponentPublicInstance } from "vue";

const multiTabRef = ref<
  ComponentPublicInstance & {
    refreshData: (id: any) => void;
  }
>();

const treeData = ref([
  { id: 1, label: "Documents", children: [{ id: 4, label: "Projects" }] },
  { id: 2, label: "Downloads", children: [{ id: 6, label: "Apps" }] },
]);

const handleNodeSelected = (node: any) => {
  const id = toRaw(node.id);
  if (multiTabRef.value) {
    multiTabRef.value.refreshData(id);
  }
};

// 统计函数
const getTreeNodeCount = () => {
  const countNodes = (nodes: any[]): number => {
    let count = 0;
    nodes.forEach((node) => {
      count++;
      if (node.children && node.children.length > 0) {
        count += countNodes(node.children);
      }
    });
    return count;
  };
  return countNodes(treeData.value);
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
</script>

<template>
  <div class="home-layout">
    <div class="home-content">
      <div class="left-panel">
        <LeftTreePanel :tree-data="treeData" @node-selected="handleNodeSelected" />
      </div>
      <div class="resizer"></div>
      <div class="right-panel">
        <MultiTabView ref="multiTabRef" />
      </div>
    </div>

    <!-- 底部统计信息 -->
    <div class="footer-stats">
      <div class="stats-container">
        <span class="stat-item">
          <span class="stat-label">项目节点:</span>
          <span class="stat-value">{{ getTreeNodeCount() }}</span>
        </span>
        <span class="stat-separator">|</span>
        <span class="stat-item">
          <span class="stat-label">活跃用户:</span>
          <span class="stat-value">3</span>
        </span>
        <span class="stat-separator">|</span>
        <span class="stat-item">
          <span class="stat-label">在线状态:</span>
          <span class="stat-value highlight">正常</span>
        </span>
        <span class="stat-separator">|</span>
        <span class="stat-item">
          <span class="stat-label">数据同步:</span>
          <span class="stat-value">已同步</span>
        </span>
        <span class="stat-separator">|</span>
        <span class="stat-item">
          <span class="stat-label">总预算:</span>
          <span class="stat-value highlight">¥15,876,543.20</span>
        </span>
        <span class="stat-separator">|</span>
        <span class="stat-item">
          <span class="stat-label">当前时间:</span>
          <span class="stat-value">{{ getCurrentTime() }}</span>
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-layout {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.home-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: 0;
}

.left-panel {
  width: 280px;
  border-right: 1px solid #e4e7ed;
}

.resizer {
  width: 5px;
  cursor: col-resize;
}

.right-panel {
  flex: 1;
  /* MultiTabView 内部已加 padding */
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
</style>
