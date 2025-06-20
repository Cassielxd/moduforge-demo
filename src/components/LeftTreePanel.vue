<script lang="ts">
import { defineComponent, ref, reactive } from "vue";
import type { PropType } from "vue";
import type { ElTree } from "element-plus";
import { ElMessage, ElMessageBox } from "element-plus";

interface Tree {
  id: number;
  label: string;
  children?: Tree[];
  remark?: string;
}

export default defineComponent({
  name: "LeftTreePanel",
  props: {
    treeData: {
      type: Array as PropType<Tree[]>,
      required: true,
    },
  },
  emits: ["update:treeData", "node-selected"],
  setup(props, { emit }) {
    const localTreeData = ref<Tree[]>(props.treeData);

    const treeRef = ref<InstanceType<typeof ElTree>>();
    const treeContextMenuVisible = ref(false);
    const treeContextMenuPosition = reactive({ x: 0, y: 0 });
    const currentTreeItem = ref<Tree | null>(null);
    const currentTreeNode = ref<any>(null);
    const editingTreeNodeId = ref<number | null>(null);
    const selectedTreeNodeId = ref<number | undefined>(undefined);
    const remarkDialogVisible = ref(false);
    const remarkContent = ref("");
    const addNodeDialogVisible = ref(false);
    const newNodeName = ref("");

    const handleTreeContextMenu = (event: MouseEvent, data: Tree, node: any) => {
      event.preventDefault();
      currentTreeItem.value = data;
      currentTreeNode.value = node;
      treeContextMenuPosition.x = event.clientX;
      treeContextMenuPosition.y = event.clientY;
      treeContextMenuVisible.value = true;
      document.addEventListener("click", closeTreeContextMenu);
    };

    const closeTreeContextMenu = () => {
      treeContextMenuVisible.value = false;
      document.removeEventListener("click", closeTreeContextMenu);
    };

    const handleTreeCommand = (command: string) => {
      if (command === "add") {
        newNodeName.value = "";
        addNodeDialogVisible.value = true;
        return;
      }
      if (!currentTreeItem.value || !currentTreeNode.value) return;
      switch (command) {
        case "edit":
          editingTreeNodeId.value = currentTreeItem.value.id;
          break;
        case "delete":
          ElMessageBox.confirm("确定要删除这个节点吗？", "警告", {
            confirmButtonText: "确定",
            cancelButtonText: "取消",
            type: "warning",
          })
            .then(() => {
              const parent = currentTreeNode.value.parent;
              const children = parent.data.children || parent.data;
              const index = children.findIndex(
                (item: Tree) => item.id === currentTreeItem.value?.id
              );
              children.splice(index, 1);
              ElMessage.success("删除成功");
              emit("update:treeData", localTreeData.value);
            })
            .catch(() => {});
          break;
        case "remark":
          remarkContent.value = currentTreeItem.value.remark || "";
          remarkDialogVisible.value = true;
          break;
      }
    };

    const handleRemarkConfirm = () => {
      if (currentTreeItem.value) {
        currentTreeItem.value.remark = remarkContent.value;
        ElMessage.success("备注已保存");
        emit("update:treeData", localTreeData.value);
      }
      remarkDialogVisible.value = false;
    };

    const handleAddNodeConfirm = () => {
      if (!newNodeName.value.trim()) {
        ElMessage.warning("请输入节点名称");
        return;
      }

      const newNode = {
        id: Date.now(),
        label: newNodeName.value.trim(),
      };

      if (currentTreeItem.value) {
        if (!currentTreeItem.value.children) {
          currentTreeItem.value.children = [];
        }
        currentTreeItem.value.children.push(newNode);
      } else {
        localTreeData.value.push(newNode);
      }

      // 设置新节点为当前选中节点
      selectedTreeNodeId.value = newNode.id;
      currentTreeItem.value = newNode;

      // 通知父组件节点已选中
      emit("node-selected", newNode);

      ElMessage.success("添加成功");
      emit("update:treeData", localTreeData.value);
      addNodeDialogVisible.value = false;
      newNodeName.value = "";
    };

    const handleTreeNodeDblClick = (data: Tree) => {
      editingTreeNodeId.value = data.id;
    };
    const finishEditTreeNode = () => {
      editingTreeNodeId.value = null;
      emit("update:treeData", localTreeData.value);
    };
    const handleTreeNodeClick = (data: Tree, node: any) => {
      currentTreeItem.value = data;
      currentTreeNode.value = node;
      selectedTreeNodeId.value = data.id;
      emit("node-selected", data);
    };

    return {
      localTreeData,
      treeRef,
      treeContextMenuVisible,
      treeContextMenuPosition,
      handleTreeContextMenu,
      handleTreeCommand,
      closeTreeContextMenu,
      editingTreeNodeId,
      finishEditTreeNode,
      handleTreeNodeDblClick,
      handleTreeNodeClick,
      selectedTreeNodeId,
      remarkDialogVisible,
      remarkContent,
      handleRemarkConfirm,
      addNodeDialogVisible,
      newNodeName,
      handleAddNodeConfirm,
    };
  },
});
</script>

<template>
  <div class="tree-panel-container">
    <div class="panel-header">
      <h3>工程项目</h3>
      <el-button type="primary" size="small" @click="handleTreeCommand('add')">
        <el-icon><Plus /></el-icon>
      </el-button>
    </div>
    <el-tree
      ref="treeRef"
      :data="localTreeData"
      node-key="id"
      :current-node-key="selectedTreeNodeId"
      default-expand-all
      :expand-on-click-node="false"
      @node-contextmenu="handleTreeContextMenu"
      @node-dblclick="handleTreeNodeDblClick"
      @node-click="handleTreeNodeClick"
      draggable
    >
      <template #default="{ data }">
        <span class="custom-tree-node">
          <template v-if="editingTreeNodeId === data.id">
            <el-input
              v-model="data.label"
              class="editable-input"
              size="small"
              autofocus
              @blur="finishEditTreeNode"
              @keyup.enter="finishEditTreeNode"
            />
          </template>
          <template v-else>
            <span @dblclick="handleTreeNodeDblClick(data)">{{ data.label }}</span>
            <el-tooltip
              v-if="data.remark"
              :content="data.remark"
              placement="right"
              effect="light"
            >
              <el-icon class="remark-icon"><ChatDotRound /></el-icon>
            </el-tooltip>
          </template>
        </span>
      </template>
    </el-tree>

    <!-- Tree Context Menu -->
    <div
      v-if="treeContextMenuVisible"
      class="custom-context-menu"
      :style="{
        left: treeContextMenuPosition.x + 'px',
        top: treeContextMenuPosition.y + 'px',
        zIndex: 2000,
      }"
    >
      <div class="context-menu-item" @click="handleTreeCommand('add')">
        <el-icon><Plus /></el-icon> 添加子节点
      </div>
      <div class="context-menu-item" @click="handleTreeCommand('edit')">
        <el-icon><Edit /></el-icon> 编辑
      </div>
      <div class="context-menu-item" @click="handleTreeCommand('remark')">
        <el-icon><ChatDotRound /></el-icon> 添加备注
      </div>
      <div class="context-menu-item" @click="handleTreeCommand('delete')">
        <el-icon><Delete /></el-icon> 删除
      </div>
    </div>

    <!-- Add Node Dialog -->
    <el-dialog
      v-model="addNodeDialogVisible"
      title="添加新节点"
      width="30%"
      :close-on-click-modal="false"
    >
      <el-form label-width="80px">
        <el-form-item label="节点名称">
          <el-input
            v-model="newNodeName"
            placeholder="请输入节点名称"
            autofocus
            @keyup.enter="handleAddNodeConfirm"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="addNodeDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleAddNodeConfirm">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- Remark Dialog -->
    <el-dialog
      v-model="remarkDialogVisible"
      title="添加备注"
      width="30%"
      :close-on-click-modal="false"
    >
      <el-input
        v-model="remarkContent"
        type="textarea"
        :rows="4"
        placeholder="请输入备注内容"
      />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="remarkDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleRemarkConfirm">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.tree-panel-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.el-tree {
  flex: 1;
  overflow-y: auto;
  padding: 8px 16px;
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
.custom-tree-node {
  flex: 1;
  display: flex;
  align-items: center;
  min-width: 0;
  gap: 8px;
}

.editable-input {
  flex: 1;
  min-width: 0;
  border: none;
  background: transparent;
  padding: 2px 0;
  margin-left: 4px;
}

.editable-input:focus {
  outline: none;
  background: #fff;
  box-shadow: 0 0 0 1px #409eff;
}

:deep(.el-tree-node__content) {
  padding: 4px 0;
  border-radius: 4px;
  margin: 2px 0;
  transition: all 0.2s;
}

:deep(.el-tree-node__content:hover) {
  background-color: #f5f7fa;
}

:deep(.el-tree-node.is-current > .el-tree-node__content) {
  background-color: #409eff;
  color: #fff;
  font-weight: 500;
}

:deep(.el-tree-node.is-current > .el-tree-node__content:hover) {
  background-color: #337ecc;
}

:deep(.el-tree-node.is-current .el-tree-node__expand-icon) {
  color: #fff;
}

:deep(.el-tree-node.is-current .remark-icon) {
  color: #fff;
}

:deep(.el-tree-node.is-current .remark-icon:hover) {
  color: #f0f9ff;
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

.remark-icon {
  color: #909399;
  font-size: 14px;
  margin-left: 4px;
  cursor: pointer;
  transition: color 0.2s;
  flex-shrink: 0;
}

.remark-icon:hover {
  color: #409eff;
}

:deep(.el-tooltip__trigger) {
  display: inline-flex;
  align-items: center;
}
</style>
