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
</script>

<template>
  <div class="home-content">
    <div class="left-panel">
      <LeftTreePanel :tree-data="treeData" @node-selected="handleNodeSelected" />
    </div>
    <div class="resizer"></div>
    <div class="right-panel">
      <MultiTabView ref="multiTabRef" />
    </div>
  </div>
</template>

<style scoped>
.home-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  height: 100%;
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
</style>
