import { ref } from 'vue';
import { useI18n } from './useI18n';

/**
 * 确认删除 composable
 * 提供了一个通用的"双击确认删除"功能
 * 
 * @param deleteFunction - 实际执行删除的异步函数，接收item id作为参数
 * @param confirmationTimeout - 确认超时时间（毫秒），默认为3500ms
 * @returns 包含删除逻辑和状态的对象
 */
export function useConfirmDelete<T = string>(
  deleteFunction: (id: T) => Promise<void>,
  confirmationTimeout: number = 3500
) {
  const { t } = useI18n();
  
  const pendingDeleteId = ref<T | null>(null);
  const deleteTimeout = ref<any>(null);

  /**
   * 执行删除操作（带确认逻辑）
   * @param id - 要删除的项目ID
   */
  async function confirmDelete(id: T) {
    clearTimeout(deleteTimeout.value);

    if (pendingDeleteId.value === id) {
      // 第二次点击，执行实际删除
      try {
        await deleteFunction(id);
        pendingDeleteId.value = null;
      } catch (error) {
        // 错误处理由调用方的deleteFunction负责
        throw error;
      }
    } else {
      // 第一次点击，进入确认状态
      pendingDeleteId.value = id;
      deleteTimeout.value = setTimeout(() => {
        pendingDeleteId.value = null;
      }, confirmationTimeout);
    }
  }

  /**
   * 检查某个项目是否处于待删除状态
   * @param id - 项目ID
   * @returns 是否处于待删除状态
   */
  function isPendingDelete(id: T): boolean {
    return pendingDeleteId.value === id;
  }

  /**
   * 获取删除按钮的图标
   * @param id - 项目ID
   * @returns 按钮图标字符串
   */
  function getDeleteIcon(id: T): string {
    return isPendingDelete(id) ? '❓' : '🗑️';
  }

  /**
   * 获取删除按钮的CSS类
   * @param id - 项目ID
   * @param baseClass - 基础CSS类名，默认为'delete-btn'
   * @returns CSS类名数组或对象
   */
  function getDeleteButtonClass(id: T, baseClass: string = 'delete-btn') {
    return [baseClass, { 'confirm-delete': isPendingDelete(id) }];
  }

  /**
   * 获取删除按钮的标题文本
   * @param id - 项目ID
   * @param confirmKey - 确认状态的翻译key
   * @param normalKey - 正常状态的翻译key
   * @returns 标题文本
   */
  function getDeleteButtonTitle(
    id: T, 
    confirmKey: string, 
    normalKey: string
  ): string {
    return isPendingDelete(id) ? t(confirmKey) : t(normalKey);
  }

  /**
   * 取消删除操作
   */
  function cancelDelete() {
    clearTimeout(deleteTimeout.value);
    pendingDeleteId.value = null;
  }

  return {
    pendingDeleteId,
    confirmDelete,
    isPendingDelete,
    getDeleteIcon,
    getDeleteButtonClass,
    getDeleteButtonTitle,
    cancelDelete
  };
}