import { ipcRequest } from "@/utils/ipc";

//ipc获取工程项目树
export const getGcxmTree = async () => {
  const result = await ipcRequest("get_gcxm_tree");
  return result;
};

//ipc 根据工程项目id获取工程项目树
export const getGcxmTreeById = async (id: string) => {
  const result = await ipcRequest("get_gcxm_tree_by_id", { id });
  return result;
};

//ipc 新增 树节点
export const addGcxmTree = async (data: any) => {
  const result = await ipcRequest("add_gcxm_tree", data);
  return result;
};

//ipc 删除 树节点

export const deleteGcxmTree = async (id: string) => {
  const result = await ipcRequest("delete_gcxm_tree", { id });
  return result;
};
