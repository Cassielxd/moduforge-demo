// 业务操作组合式函数
import { ref, nextTick } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import type { TreeTableData } from "./useFbfxData";

export function useFbfxActions(
  tableTreeData: any,
  mainTabulatorComposable: any
) {
  const currentTableItem = ref<TreeTableData | null>(null);
  const currentRowKey = ref<string | number | null>(null);

  // 颜色变化处理函数
  const handleColorChange = (id: string, color: string) => {
    // 在原始数据中更新颜色
    const updateColor = (items: TreeTableData[]): void => {
      for (const item of items) {
        if (String(item.id) === id) {
          item.color = color;
          return;
        }
        if (item.children) {
          updateColor(item.children);
        }
      }
    };

    updateColor(tableTreeData.value);
    ElMessage.success(`已设置颜色：${color}`);
  };

  // 行点击处理函数
  const handleRowClick = (data: any) => {
    currentRowKey.value = data.id;
    console.log("选中行:", data);
  };

  // 添加行
  const handleAddRow = (currentRow?: TreeTableData) => {
    const newRow: TreeTableData = {
      id: Date.now(),
      name: "新文件",
      type: "file",
      size: "0KB",
      date: new Date().toISOString().split("T")[0],
    };

    if (currentRow) {
      // 在指定行的下一行插入新行
      const insertAfter = (
        data: TreeTableData[],
        targetId: number | string
      ): boolean => {
        for (let i = 0; i < data.length; i++) {
          if (data[i].id === targetId) {
            // 在当前位置的下一行插入
            data.splice(i + 1, 0, newRow);
            return true;
          }
          // 如果有子节点，递归查找
          if (data[i].children && insertAfter(data[i].children!, targetId)) {
            return true;
          }
        }
        return false;
      };

      if (!insertAfter(tableTreeData.value, currentRow.id)) {
        // 如果没有找到指定行，就添加到末尾
        tableTreeData.value.push(newRow);
      }
      ElMessage.success("在选中行下方添加成功");
    } else {
      // 没有指定行时，添加到末尾
      tableTreeData.value.push(newRow);
      ElMessage.success("添加成功");
    }

    // 选中新添加的行
    nextTick(() => {
      setCurrentRow(newRow.id);
    });
  };

  // 添加子项
  const handleAddChild = (parentRow: TreeTableData) => {
    const newChild: TreeTableData = {
      id: Date.now(),
      name: "新子项",
      type: "file",
      size: "0KB",
      date: new Date().toISOString().split("T")[0],
    };

    console.log("添加子项到父行:", parentRow);

    if (!parentRow.children) {
      parentRow.children = [];
    }
    parentRow.children.push(newChild);

    ElMessage.success("添加子项成功");
  };

  // 编辑行
  const handleEditRow = (row: TreeTableData) => {
    ElMessage.info("编辑功能，可以双击单元格编辑");
  };

  // 删除行
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

  // 复制行
  const handleCopyRow = (row: TreeTableData) => {
    const newItem: TreeTableData = {
      ...row,
      id: Date.now(),
      name: `${row.name} (复制)`,
      children: undefined, // 复制时不包含子项
    };
    tableTreeData.value.push(newItem);

    ElMessage.success("复制成功");

    // 选中复制的新行
    nextTick(() => {
      setCurrentRow(newItem.id);
    });
  };

  // 设置当前行
  const setCurrentRow = (rowId: string | number) => {
    currentRowKey.value = rowId;
    // 使用组合式函数选择行
    mainTabulatorComposable.selectRow(String(rowId));
  };

  // 子表格事件处理
  const handleSubTableAddRow = () => {
    ElMessage.info("子表格新增功能");
  };

  const handleSubTableEditRow = (row: any) => {
    ElMessage.info("子表格编辑功能，可以双击单元格编辑");
  };

  const handleSubTableDeleteRow = (row: any) => {
    ElMessage.info("子表格删除功能");
  };

  const handleSubTableCopyRow = (row: any) => {
    ElMessage.info("子表格复制功能");
  };

  return {
    // 状态
    currentTableItem,
    currentRowKey,
    // 主表格操作
    handleColorChange,
    handleRowClick,
    handleAddRow,
    handleAddChild,
    handleEditRow,
    handleDeleteRow,
    handleCopyRow,
    setCurrentRow,
    // 子表格操作
    handleSubTableAddRow,
    handleSubTableEditRow,
    handleSubTableDeleteRow,
    handleSubTableCopyRow,
  };
}
