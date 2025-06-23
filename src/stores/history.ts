import { getHistory } from "@/api/common";
import { defineStore } from "pinia";
import { ref } from "vue";

// 历史记录项类型
export interface HistoryItem {
  state_version: number;
  description: string;
  timestamp: string;
  type: "创建" | "修改" | "删除";
}

// 全局历史记录 store
export const useHistoryStore = defineStore("history", () => {
  // 历史记录列表，初始可为空
  const historyList = ref<HistoryItem[]>([]);

  // 添加历史记录
  function addHistory(item: HistoryItem) {
    historyList.value.unshift(item); // 新记录放前面
  }

  // 清空历史记录
  function clearHistory() {
    historyList.value = [];
  }
  // 整体替换历史记录列表
  function setHistoryList(list: HistoryItem[]) {
    historyList.value = list;
  }
  function refreshHistory(id: string) {
    getHistory({ editor_name: id }).then((res: any) => {
      historyList.value = res.data as HistoryItem[];
    });
  }
  return {
    historyList,
    addHistory,
    clearHistory,
    setHistoryList,
    refreshHistory,
  };
});
