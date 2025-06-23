<script lang="ts">
import { defineComponent, ref, reactive, onMounted, onUnmounted, nextTick, watch } from "vue";
import type { PropType } from "vue";
import { ElMessage, ElMessageBox, ElDialog, ElButton, ElColorPicker, ElIcon } from "element-plus";
// @ts-ignore
import { Tabulator } from 'tabulator-tables';
// @ts-ignore
import type { RowComponent, CellComponent, TabulatorConfig } from 'tabulator-tables';
import 'tabulator-tables/dist/css/tabulator.min.css';
import { Plus, Edit, Delete, CopyDocument, Brush, Folder, Document } from '@element-plus/icons-vue';

// --- Data Interfaces ---
interface TableItem {
  id: number | string;
  [key: string]: any;
}

export interface TableColumn {
  prop: string;
  label: string;
  minWidth?: number;
  width?: number;
  align?: "left" | "center" | "right";
  type?: "date";
}

export default defineComponent({
  name: "RightTablePanel",
  components: {
    ElDialog,
    ElButton,
    ElColorPicker,
    ElIcon,
    Plus,
    Edit,
    Delete,
    CopyDocument,
    Brush,
    Folder,
    Document,
  },
  props: {
    tableData: {
      type: Array as PropType<TableItem[]>,
      required: true,
    },
    tableColumns: {
      type: Array as PropType<TableColumn[]>,
      required: true,
    },
    isTreeTable: {
      type: Boolean,
      default: false,
    },
  },
  emits: [
    "update:tableData",
    "add-row",
    "add-child",
    "edit-row",
    "delete-row",
    "copy-row",
  ],
  setup(props, { emit, expose }) {
    const localTableData = ref<TableItem[]>(props.tableData);
    const tableRef = ref<HTMLElement>();
    const tabulator = ref<Tabulator | null>(null);

    const tableContextMenuVisible = ref(false);
    const tableContextMenuPosition = reactive({ x: 0, y: 0 });
    const currentTableItem = ref<TableItem | null>(null);
    const currentRowKey = ref<string | number | null>(null);

    // 颜色选择相关
    const colorDialogVisible = ref(false);
    const colorValue = ref("#409EFF");

    // 添加编辑标志
    const isEditing = ref(false);

    // 初始化 Tabulator
    const initTabulator = () => {
      if (!tableRef.value) return;

      // 转换列配置
      const columns = [
        // 如果是树形表格，添加树形列
        ...(props.isTreeTable ? [{
          title: "",
          field: "tree_control",
          width: 150,
          headerSort: false,
          formatter: "tree",
          headerClick: false,
          cellClick: false,
        }] : [{
          title: "",
          width: 50,
          headerSort: false,
          formatter: () => ""
        }]),
        // 数据列
        ...props.tableColumns.map(col => ({
          title: col.label,
          field: col.prop,
          width: col.width || col.minWidth || 120,
          headerSort: true,
          editor: "input",
          formatter: (cell: CellComponent) => {
            const data = cell.getRow().getData();
            const value = data[col.prop];

            if (props.isTreeTable && col.prop === 'name') {
              const iconHtml = data.type === 'folder'
                ? '<i class="folder-icon">📁</i>'
                : '<i class="file-icon">📄</i>';
              return `<div class="tree-name-cell">${iconHtml}<span>${value || ''}</span></div>`;
            }

            if (props.isTreeTable && col.prop === 'type') {
              const typeText = data.type === 'folder' ? '文件夹' : '文件';
              const typeClass = data.type === 'folder' ? 'folder-tag' : 'file-tag';
              return `<span class="type-tag ${typeClass}">${typeText}</span>`;
            }

            if (col.prop === "size" && data.type === "folder") {
              return "-";
            }

            return value || '';
          },
          cellEdited: (cell: CellComponent) => {
            const row = cell.getRow();
            const data = row.getData();
            isEditing.value = true; // 设置编辑标志
            localTableData.value = tabulator.value?.getData() || [];
            emit("update:tableData", localTableData.value);

            // 延迟重置编辑标志
            setTimeout(() => {
              isEditing.value = false;
            }, 100);
          }
        }))
      ];

      // 配置 Tabulator
      const config: any = {
        data: localTableData.value,
        columns: columns,
        layout: "fitColumns",
        height: "100%",
        rowContextMenu: [
          {
            label: "添加行",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              emit("add-row", currentTableItem.value);
            }
          },
          {
            label: "编辑",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              emit("edit-row", currentTableItem.value);
            }
          },
          {
            label: "删除",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              emit("delete-row", currentTableItem.value);
            }
          },
          {
            label: "复制",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              emit("copy-row", currentTableItem.value);
            }
          },
          {
            label: "添加子项",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              debugger;
              emit("add-child", currentTableItem.value, row);
            }
          },
          {
            label: "设置边框颜色",
            action: (e: Event, row: RowComponent) => {
              currentTableItem.value = row.getData() as TableItem;
              openColorDialog();
            }
          }
        ],
        rowFormatter: (row: RowComponent) => {
          const data = row.getData();
          if (data.color && data.color.trim()) {
            const element = row.getElement();
            element.style.border = `2px solid ${data.color}`;
            element.classList.add('colored-border');
          }
        }
      };

      // 如果是树形表格，添加树形配置
      if (props.isTreeTable) {
        config.dataTree = true;
        config.dataTreeChildField = "children";
        config.dataTreeStartExpanded = false;
      }

      tabulator.value = new Tabulator(tableRef.value, config);

      // 监听行选择
      tabulator.value.on("rowClick", (e: Event, row: RowComponent) => {
        currentRowKey.value = row.getData().id;

        // 清除所有行的选中状态
        tabulator.value?.getRows().forEach((r: RowComponent) => {
          r.getElement().classList.remove('row-selected');
        });

        // 为当前选中行添加选中样式
        row.getElement().classList.add('row-selected');
      });
    };

    const handleTableContextMenu = (row: TableItem, column: any, event: MouseEvent) => {
      event.preventDefault();
      currentTableItem.value = row;
      tableContextMenuPosition.x = event.clientX;
      tableContextMenuPosition.y = event.clientY;
      tableContextMenuVisible.value = true;
      document.addEventListener("click", closeTableContextMenu);
    };

    const closeTableContextMenu = () => {
      tableContextMenuVisible.value = false;
      document.removeEventListener("click", closeTableContextMenu);
    };

    const handleTableCommand = (command: string) => {
      if (!currentTableItem.value) return;

      switch (command) {
        case "add-row":
          emit("add-row", currentTableItem.value);
          break;
        case "edit":
          emit("edit-row", currentTableItem.value);
          break;
        case "delete":
          emit("delete-row", currentTableItem.value);
          break;
        case "copy":
          emit("copy-row", currentTableItem.value);
          break;
        case "add-child":
          emit("add-child", currentTableItem.value);
          break;
      }
      tableContextMenuVisible.value = false;
    };

    const handleAddTableRow = () => {
      emit("add-row");
    };

    const openColorDialog = () => {
      colorValue.value = currentTableItem.value?.color || "#409EFF";
      colorDialogVisible.value = true;
      tableContextMenuVisible.value = false;
    };

    const handleColorSubmit = () => {
      if (!colorValue.value) {
        ElMessage.warning("请选择颜色");
        return;
      }
      if (currentTableItem.value && tabulator.value) {
        // 更新 Tabulator 中的数据
        const rows = tabulator.value.getRows();
        const targetRow = rows.find((row: RowComponent) => row.getData().id === currentTableItem.value!.id);
        if (targetRow) {
          targetRow.update({ ...targetRow.getData(), color: colorValue.value });
          localTableData.value = tabulator.value.getData();
          emit("update:tableData", localTableData.value);
        }
      }
      colorDialogVisible.value = false;
    };

    // 设置当前选中行的方法，供父组件调用
    const setCurrentRow = (rowId: string | number) => {
      currentRowKey.value = rowId;
      if (tabulator.value) {
        // 使用 nextTick 确保数据已更新并渲染完成
        nextTick(() => {
          // 添加一个小延迟，确保数据监听器完成更新
          setTimeout(() => {
            if (!tabulator.value) return;

            const rows = tabulator.value.getRows();

            // 清除所有行的选中状态
            rows.forEach((r: RowComponent) => {
              r.getElement().classList.remove('row-selected');
            });

            const targetRow = rows.find((row: RowComponent) => row.getData().id === rowId);
            if (targetRow) {
              targetRow.select();
              // 添加自定义选中样式
              targetRow.getElement().classList.add('row-selected');

              // 如果是树形表格，确保目标行可见（滚动到视图中）
              if (props.isTreeTable) {
                try {
                  targetRow.getElement().scrollIntoView({
                    behavior: 'smooth',
                    block: 'nearest'
                  });
                } catch (error) {
                  console.warn('滚动到目标行失败:', error);
                }
              }
            } else {
              console.log('未找到要选中的行:', rowId);
              console.log('可用的行ID:', rows.map((r: RowComponent) => r.getData().id));
            }
          }, 100); // 100ms 延迟确保数据更新完成
        });
      }
    };

    // 简化：移除展开功能，只保留基本的树形表格显示

    // 更新数据的方法
    const updateData = (newData: TableItem[]) => {
      const previousSelectedId = currentRowKey.value;
      localTableData.value = newData;
      if (tabulator.value) {
        tabulator.value.setData(newData);

        // 数据更新后恢复选中状态
        if (previousSelectedId) {
          nextTick(() => {
            setCurrentRow(previousSelectedId);
          });
        }
      }
    };

    // 刷新表格数据（强制重新渲染）
    const refreshTable = () => {
      if (tabulator.value) {
        tabulator.value.setData(localTableData.value);
      }
    };

    onMounted(() => {
      nextTick(() => {
        initTabulator();
      });
    });

    onUnmounted(() => {
      if (tabulator.value) {
        tabulator.value.destroy();
      }
    });

    // 监听数据变化
    watch(() => props.tableData, (newData) => {
      console.log('props.tableData 变化:', newData);
      localTableData.value = newData;
      if (tabulator.value) {
        // 检查是否是编辑操作导致的数据变化
        const currentData = tabulator.value.getData();
        const isEditOperation = isEditing.value || (currentData.length === newData.length);

        if (isEditOperation) {
          // 如果是编辑操作，只更新数据，不重建表格
          // 保存当前的展开状态
          const expandedRows = tabulator.value.getRows().filter((row: RowComponent) => row.isTreeExpanded()).map((row: RowComponent) => row.getData().id);

          // 更新数据
          tabulator.value.setData(newData);

          // 恢复展开状态
          nextTick(() => {
            if (tabulator.value) {
              expandedRows.forEach((id: string | number) => {
                const row = tabulator.value!.getRows().find((r: RowComponent) => r.getData().id === id);
                if (row && row.getData().children && row.getData().children.length > 0) {
                  row.treeExpand();
                }
              });
            }
          });
        } else {
          // 如果是结构变化（添加/删除行），才重建表格
          tabulator.value.setData([]);
          nextTick(() => {
            if (tabulator.value) {
              tabulator.value.setData(newData);
            }
          });
        }
      }
    }, { deep: true, immediate: true });

    // 清除选中状态的方法
    const clearSelection = () => {
      currentRowKey.value = null;
      if (tabulator.value) {
        tabulator.value.getRows().forEach((r: RowComponent) => {
          r.getElement().classList.remove('row-selected');
        });
        tabulator.value.deselectRow();
      }
    };

    // ========== 新增方法 ==========

    // 获取当前选中行数据
    const getCurrentRow = () => {
      if (!tabulator.value || !currentRowKey.value) return null;
      const rows = tabulator.value.getRows();
      return rows.find((row: RowComponent) => row.getData().id === currentRowKey.value)?.getData() || null;
    };

    // 获取所有行数据
    const getAllData = () => {
      return tabulator.value?.getData() || [];
    };

    // 根据ID获取行数据
    const getRowById = (id: string | number) => {
      if (!tabulator.value) return null;
      const rows = tabulator.value.getRows();
      return rows.find((row: RowComponent) => row.getData().id === id)?.getData() || null;
    };

    // 根据条件查找行
    const findRows = (predicate: (row: TableItem) => boolean) => {
      if (!tabulator.value) return [];
      const rows = tabulator.value.getRows();
      return rows
        .map((row: RowComponent) => row.getData() as TableItem)
        .filter(predicate);
    };

    // 更新指定行的数据
    const updateRow = (id: string | number, newData: Partial<TableItem>) => {
      if (!tabulator.value) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow) {
        targetRow.update({ ...targetRow.getData(), ...newData });
        localTableData.value = tabulator.value.getData();
        emit("update:tableData", localTableData.value);
        return true;
      }
      return false;
    };

    // 删除指定行
    const deleteRow = (id: string | number) => {
      if (!tabulator.value) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow) {
        // 使用类型断言来访问 delete 方法
        (targetRow as any).delete();
        localTableData.value = tabulator.value.getData();
        emit("update:tableData", localTableData.value);
        return true;
      }
      return false;
    };

    // 添加新行
    const addRow = (rowData: TableItem) => {
      if (!tabulator.value) return false;
      // 使用类型断言来访问 addRow 方法
      (tabulator.value as any).addRow(rowData);
      localTableData.value = tabulator.value.getData();
      emit("update:tableData", localTableData.value);
      return true;
    };

    // 树形表格相关方法
    const expandAll = () => {
      if (!tabulator.value || !props.isTreeTable) return;
      const rows = tabulator.value.getRows();
      rows.forEach((row: RowComponent) => {
        if (row.getData().children && row.getData().children.length > 0) {
          row.treeExpand();
        }
      });
    };

    const collapseAll = () => {
      if (!tabulator.value || !props.isTreeTable) return;
      const rows = tabulator.value.getRows();
      rows.forEach((row: RowComponent) => {
        if (row.isTreeExpanded()) {
          row.treeCollapse();
        }
      });
    };

    const expandRow = (id: string | number) => {
      if (!tabulator.value || !props.isTreeTable) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow && targetRow.getData().children && targetRow.getData().children.length > 0) {
        targetRow.treeExpand();
        return true;
      }
      return false;
    };

    const collapseRow = (id: string | number) => {
      if (!tabulator.value || !props.isTreeTable) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow && targetRow.isTreeExpanded()) {
        targetRow.treeCollapse();
        return true;
      }
      return false;
    };

    const isRowExpanded = (id: string | number) => {
      if (!tabulator.value || !props.isTreeTable) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      return targetRow ? targetRow.isTreeExpanded() : false;
    };

    // 获取展开的行ID列表
    const getExpandedRows = () => {
      if (!tabulator.value || !props.isTreeTable) return [];
      const rows = tabulator.value.getRows();
      return rows.filter((row: RowComponent) => row.isTreeExpanded()).map((row: RowComponent) => row.getData().id);
    };

    // 设置展开状态
    const setExpandedRows = (ids: (string | number)[]) => {
      if (!tabulator.value || !props.isTreeTable) return;
      const rows = tabulator.value.getRows();

      // 先收起所有行
      rows.forEach((row: RowComponent) => {
        if (row.isTreeExpanded()) {
          row.treeCollapse();
        }
      });

      // 展开指定的行
      ids.forEach((id: string | number) => {
        const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
        if (targetRow && targetRow.getData().children && targetRow.getData().children.length > 0) {
          targetRow.treeExpand();
        }
      });
    };

    // 排序相关方法
    const sortBy = (field: string, dir: 'asc' | 'desc' = 'asc') => {
      if (!tabulator.value) return;
      (tabulator.value as any).setSort(field, dir);
    };

    const clearSort = () => {
      if (!tabulator.value) return;
      (tabulator.value as any).clearSort();
    };

    // 过滤相关方法
    const setFilter = (field: string, type: string, value: any) => {
      if (!tabulator.value) return;
      (tabulator.value as any).setFilter(field, type, value);
    };

    const clearFilter = () => {
      if (!tabulator.value) return;
      (tabulator.value as any).clearFilter();
    };

    // 分页相关方法
    const setPage = (page: number) => {
      if (!tabulator.value) return;
      (tabulator.value as any).setPage(page);
    };

    const getCurrentPage = () => {
      if (!tabulator.value) return 1;
      return (tabulator.value as any).getPage();
    };

    const getPageSize = () => {
      if (!tabulator.value) return 10;
      return (tabulator.value as any).getPageSize();
    };

    const setPageSize = (size: number) => {
      if (!tabulator.value) return;
      (tabulator.value as any).setPageSize(size);
    };

    // 选择相关方法
    const selectRow = (id: string | number) => {
      if (!tabulator.value) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow) {
        targetRow.select();
        return true;
      }
      return false;
    };

    const deselectRow = (id: string | number) => {
      if (!tabulator.value) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow) {
        (targetRow as any).deselect();
        return true;
      }
      return false;
    };

    const getSelectedRows = () => {
      if (!tabulator.value) return [];
      return (tabulator.value as any).getSelectedRows().map((row: RowComponent) => row.getData());
    };

    const selectAll = () => {
      if (!tabulator.value) return;
      (tabulator.value as any).selectRow();
    };

    const deselectAll = () => {
      if (!tabulator.value) return;
      tabulator.value.deselectRow();
    };

    // 滚动相关方法
    const scrollToRow = (id: string | number, position: 'start' | 'center' | 'end' = 'center') => {
      if (!tabulator.value) return false;
      const rows = tabulator.value.getRows();
      const targetRow = rows.find((row: RowComponent) => row.getData().id === id);
      if (targetRow) {
        targetRow.getElement().scrollIntoView({
          behavior: 'smooth',
          block: position
        });
        return true;
      }
      return false;
    };

    const scrollToTop = () => {
      if (!tabulator.value) return;
      const tableElement = (tabulator.value as any).getElement();
      if (tableElement) {
        tableElement.scrollTop = 0;
      }
    };

    const scrollToBottom = () => {
      if (!tabulator.value) return;
      const tableElement = (tabulator.value as any).getElement();
      if (tableElement) {
        tableElement.scrollTop = tableElement.scrollHeight;
      }
    };

    // 导出相关方法
    const exportToCSV = (filename?: string) => {
      if (!tabulator.value) return;
      (tabulator.value as any).download('csv', filename || 'table-data.csv');
    };

    const exportToJSON = (filename?: string) => {
      if (!tabulator.value) return;
      (tabulator.value as any).download('json', filename || 'table-data.json');
    };

    const exportToPDF = (filename?: string) => {
      if (!tabulator.value) return;
      (tabulator.value as any).download('pdf', filename || 'table-data.pdf');
    };

    // 表格状态方法
    const getTableState = () => {
      if (!tabulator.value) return null;
      return {
        currentPage: (tabulator.value as any).getPage(),
        pageSize: (tabulator.value as any).getPageSize(),
        sort: (tabulator.value as any).getSorters(),
        filter: (tabulator.value as any).getFilters(),
        selectedRows: (tabulator.value as any).getSelectedRows().map((row: RowComponent) => row.getData().id),
        expandedRows: props.isTreeTable ? getExpandedRows() : [],
        currentRow: currentRowKey.value
      };
    };

    const setTableState = (state: any) => {
      if (!tabulator.value) return;

      if (state.sort) {
        (tabulator.value as any).setSort(state.sort);
      }

      if (state.filter) {
        (tabulator.value as any).setFilter(state.filter);
      }

      if (state.pageSize) {
        (tabulator.value as any).setPageSize(state.pageSize);
      }

      if (state.currentPage) {
        (tabulator.value as any).setPage(state.currentPage);
      }

      if (state.expandedRows && props.isTreeTable) {
        setExpandedRows(state.expandedRows);
      }

      if (state.currentRow) {
        setCurrentRow(state.currentRow);
      }
    };

    // 重新初始化表格
    const reinitializeTable = () => {
      if (tabulator.value) {
        tabulator.value.destroy();
      }
      nextTick(() => {
        initTabulator();
      });
    };

    // 获取表格统计信息
    const getTableStats = () => {
      if (!tabulator.value) return null;
      const rows = tabulator.value.getRows();
      return {
        totalRows: rows.length,
        visibleRows: rows.filter((row: RowComponent) => (row as any).isVisible()).length,
        selectedRows: (tabulator.value as any).getSelectedRows().length,
        expandedRows: props.isTreeTable ? getExpandedRows().length : 0,
        currentPage: (tabulator.value as any).getPage(),
        totalPages: (tabulator.value as any).getPageMax()
      };
    };

    // 暴露方法给父组件
    expose({
      setCurrentRow,
      updateData,
      clearSelection,
      refreshTable,
      getCurrentRow,
      getAllData,
      getRowById,
      findRows,
      updateRow,
      deleteRow,
      addRow,
      expandAll,
      collapseAll,
      expandRow,
      collapseRow,
      isRowExpanded,
      getExpandedRows,
      setExpandedRows,
      sortBy,
      clearSort,
      setFilter,
      clearFilter,
      setPage,
      getCurrentPage,
      getPageSize,
      setPageSize,
      selectRow,
      deselectRow,
      getSelectedRows,
      selectAll,
      deselectAll,
      scrollToRow,
      scrollToTop,
      scrollToBottom,
      exportToCSV,
      exportToJSON,
      exportToPDF,
      getTableState,
      setTableState,
      reinitializeTable,
      getTableStats,
    });

    return {
      localTableData,
      tableRef,
      tabulator,
      tableContextMenuVisible,
      tableContextMenuPosition,
      handleTableContextMenu,
      handleTableCommand,
      closeTableContextMenu,
      handleAddTableRow,
      colorDialogVisible,
      colorValue,
      openColorDialog,
      handleColorSubmit,
      currentRowKey,
      setCurrentRow,
      clearSelection,
      refreshTable,
    };
  },
});
</script>

<template>
  <div class="table-panel-container">
    <el-dialog v-model="colorDialogVisible" title="设置边框颜色" width="300px">
      <el-color-picker v-model="colorValue" />
      <template #footer>
        <el-button @click="colorDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleColorSubmit">确定</el-button>
      </template>
    </el-dialog>

    <!-- Tabulator 表格容器 -->
    <div ref="tableRef" class="tabulator-table"></div>
  </div>
</template>

<style scoped>
.table-panel-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.tabulator-table {
  flex: 1;
  height: 100%;
}

/* Tabulator 自定义样式 */
:deep(.tabulator) {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  background: #fff;
}

:deep(.tabulator .tabulator-header) {
  background: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
}

:deep(.tabulator .tabulator-header .tabulator-col) {
  background: #f5f7fa;
  border-right: 1px solid #e4e7ed;
  font-weight: 600;
  color: #606266;
}

:deep(.tabulator .tabulator-row) {
  border-bottom: 1px solid #e4e7ed;
  position: relative;
}

:deep(.tabulator .tabulator-row .tabulator-cell) {
  border-right: 1px solid #e4e7ed;
  padding: 8px 12px;
}

/* 有颜色边框的行，调整单元格边框 */
:deep(.tabulator .tabulator-row.colored-border .tabulator-cell) {
  border-right: none !important;
}

:deep(.tabulator .tabulator-row:hover) {
  background-color: #f5f7fa;
}

:deep(.tabulator .tabulator-row.tabulator-selected) {
  background-color: #ecf5ff;
}

/* 自定义行选中样式 */
:deep(.tabulator .tabulator-row.row-selected) {
  background-color: #e6f7ff !important;
  border-left: 1px solid #1890ff !important;
  position: relative;
}

/* 选中行悬停效果 */
:deep(.tabulator .tabulator-row.row-selected:hover) {
  background-color: #bae7ff !important;
}

/* 选中行的单元格样式 */
:deep(.tabulator .tabulator-row.row-selected .tabulator-cell) {
  border-color: #91d5ff;
  font-weight: 500;
}

/* 确保有颜色边框的选中行样式正确叠加 */
:deep(.tabulator .tabulator-row.row-selected.colored-border) {
  box-shadow: inset 0 0 0 2px #1890ff, 0 0 0 2px currentColor;
}

/* 树形表格样式 */
.tree-name-cell {
  display: flex;
  align-items: center;
  gap: 8px;
}

.folder-icon {
  color: #e6a23c;
  font-size: 16px;
}

.file-icon {
  color: #909399;
  font-size: 16px;
}

.type-tag {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.folder-tag {
  background: #fdf6ec;
  color: #e6a23c;
  border: 1px solid #f5dab1;
}

.file-tag {
  background: #f4f4f5;
  color: #909399;
  border: 1px solid #d3d4d6;
}

/* 树形控制列样式 */
:deep(.tabulator .tabulator-cell[data-field="tree_control"]) {
  padding: 4px 8px;
  text-align: center;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control) {
  display: inline-block;
  width: 16px;
  height: 16px;
  line-height: 16px;
  text-align: center;
  cursor: pointer;
  border-radius: 2px;
  transition: background-color 0.2s;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control:hover) {
  background-color: #f0f0f0;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control .tabulator-tree-control-expand) {
  color: #409eff;
  font-weight: bold;
}

:deep(.tabulator .tabulator-cell[data-field="tree_control"] .tabulator-tree-control .tabulator-tree-control-collapse) {
  color: #409eff;
  font-weight: bold;
}

/* 右键菜单样式覆盖 */
:deep(.tabulator-menu) {
  background: #fff;
  border: 1px solid #e4e7ed;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  padding: 4px 0;
  font-size: 14px;
}

:deep(.tabulator-menu .tabulator-menu-item) {
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.2s;
}

:deep(.tabulator-menu .tabulator-menu-item:hover) {
  background-color: #f5f7fa;
}

/* 有边框颜色的行样式 */
:deep(.tabulator .tabulator-row.colored-border) {
  border-width: 2px !important;
  border-style: solid !important;
  margin: 1px 0 !important;
  /* 防止边框重叠 */
}

/* 确保边框颜色在悬停时也保持 */
:deep(.tabulator .tabulator-row.colored-border:hover) {
  border-width: 2px !important;
  border-style: solid !important;
}
</style>
