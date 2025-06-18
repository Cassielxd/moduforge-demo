import { defineStore } from "pinia";
import { ref } from "vue";
import {
  register,
  unregister,
  isRegistered,
  unregisterAll,
} from "@tauri-apps/plugin-global-shortcut";

export interface GlobalShortcut {
  id: string;
  name: string;
  shortcut: string;
  enabled: boolean;
  handler: () => void;
}

export const useGlobalShortcutsStore = defineStore("globalShortcuts", () => {
  const shortcuts = ref<GlobalShortcut[]>([]);
  const isInitialized = ref(false);

  // 初始化全局快捷键
  const initialize = async () => {
    if (isInitialized.value) return;

    // 注册默认快捷键
    const defaultShortcuts: GlobalShortcut[] = [
      {
        id: "new-file",
        name: "新建文件",
        shortcut: "Ctrl+N",
        enabled: true,
        handler: () => {
          console.log("新建文件");
          // TODO: 实现新建文件功能
        },
      },
      {
        id: "save-file",
        name: "保存文件",
        shortcut: "Ctrl+S",
        enabled: true,
        handler: () => {
          console.log("保存文件");
          // TODO: 实现保存文件功能
        },
      },
      {
        id: "open-file",
        name: "打开文件",
        shortcut: "Ctrl+O",
        enabled: true,
        handler: () => {
          console.log("打开文件");
          // TODO: 实现打开文件功能
        },
      },
    ];

    for (const shortcut of defaultShortcuts) {
      await register(shortcut.shortcut, shortcut.handler);
      shortcuts.value.push(shortcut);
    }

    isInitialized.value = true;
  };

  // 注册单个快捷键
  const registerShortcut = async (shortcut: GlobalShortcut) => {
    try {
      await register(shortcut.shortcut, shortcut.handler);
      shortcuts.value.push(shortcut);
    } catch (error) {
      console.error("注册快捷键失败:", error);
      throw error;
    }
  };

  // 更新快捷键
  const updateShortcut = async (id: string, newShortcut: string) => {
    const shortcut = shortcuts.value.find((s) => s.id === id);
    if (!shortcut) return;

    try {
      // 注销旧的快捷键
      await unregister(shortcut.shortcut);
      // 注册新的快捷键
      await register(newShortcut, shortcut.handler);
      // 更新快捷键
      shortcut.shortcut = newShortcut;
    } catch (error) {
      console.error("更新快捷键失败:", error);
      throw error;
    }
  };

  // 切换快捷键启用状态
  const toggleShortcut = async (id: string) => {
    const shortcut = shortcuts.value.find((s) => s.id === id);
    if (!shortcut) return;

    try {
      if (shortcut.enabled) {
        await unregister(shortcut.shortcut);
      } else {
        await register(shortcut.shortcut, shortcut.handler);
      }
      shortcut.enabled = !shortcut.enabled;
    } catch (error) {
      console.error("切换快捷键状态失败:", error);
      throw error;
    }
  };

  const addShortcut = async (shortcut: GlobalShortcut) => {
    try {
      await register(shortcut.shortcut, shortcut.handler);
      shortcuts.value.push(shortcut);
    } catch (error) {
      console.error("添加快捷键失败:", error);
      throw error;
    }
  };

  const removeShortcut = async (id: string) => {
    const shortcut = shortcuts.value.find((s) => s.id === id);
    if (!shortcut) return;

    try {
      await unregister(shortcut.shortcut);
      shortcuts.value = shortcuts.value.filter((s) => s.id !== id);
    } catch (error) {
      console.error("删除快捷键失败:", error);
      throw error;
    }
  };

  // 注销所有快捷键
  const unregisterAllShortcuts = async () => {
    try {
      await unregisterAll();
      shortcuts.value = [];
      isInitialized.value = false;
    } catch (error) {
      console.error("注销所有快捷键失败:", error);
      throw error;
    }
  };

  return {
    shortcuts,
    isInitialized,
    initialize,
    registerShortcut,
    updateShortcut,
    toggleShortcut,
    addShortcut,
    removeShortcut,
    unregisterAll: unregisterAllShortcuts,
  };
});
