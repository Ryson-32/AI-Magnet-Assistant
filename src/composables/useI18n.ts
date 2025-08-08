import { ref, computed, watch } from 'vue'
import { useI18n as vueUseI18n } from 'vue-i18n'
import { getI18n, SUPPORTED_LOCALES, type SupportedLocale, loadLocaleMessages } from '../i18n/index'
import type { TranslationKey, TranslationParams } from '../i18n/types'
import { AVAILABLE_LANGUAGES } from '../i18n/types'
import { invoke } from '@tauri-apps/api/core'

// 全局语言状态
const currentLocale = ref<SupportedLocale>('zh-CN')

/**
 * 增强的 i18n 组合式函数
 */
export function useI18n() {
  const { t, d, n, locale, ...rest } = vueUseI18n()
  
  // 类型安全的翻译函数
  const translate = (key: TranslationKey, params?: TranslationParams): string => {
    return t(key, params || {}) as string
  }
  
  // 智能复数处理
  const plural = (key: TranslationKey, count: number, params?: TranslationParams): string => {
    return t(key, { count, ...(params || {}) }) as string
  }
  
  // 带回退的安全翻译
  const safeTranslate = (key: TranslationKey, fallback: string, params?: TranslationParams): string => {
    try {
      const translation = t(key, params || {}) as string
      return translation !== key ? translation : fallback
    } catch {
      return fallback
    }
  }

  // HTML内容翻译（保持换行）
  const translateHtml = (key: TranslationKey, params?: TranslationParams): string => {
    const translated = t(key, params || {}) as string
    return translated.replace(/\n/g, '<br/>')
  }

  return {
    ...rest,
    t: translate,
    plural,
    safeTranslate,
    translateHtml,
    locale,
    d,
    n
  }
}

/**
 * 语言切换组合式函数
 */
export function useLocale() {
  const i18n = getI18n()

  // 当前语言
  const locale = computed({
    get: () => currentLocale.value,
    set: (newLocale: SupportedLocale) => {
      setLocale(newLocale)
    }
  })

  // 可用语言列表
  const availableLocales = computed(() => AVAILABLE_LANGUAGES)

  // 当前语言信息
  const currentLanguageInfo = computed(() => {
    return AVAILABLE_LANGUAGES.find(lang => lang.code === currentLocale.value) || {
      name: '未知',
      code: 'zh-CN',
      label: '简体中文'
    }
  })

  // 设置语言
  const setLocale = async (newLocale: SupportedLocale) => {
    if (!SUPPORTED_LOCALES.includes(newLocale)) {
      console.warn(`Unsupported locale: ${newLocale}`)
      return
    }

    try {
      // 1. 动态加载语言包（如果尚未加载）
      if (!i18n.global.availableLocales.includes(newLocale)) {
        const messages = await loadLocaleMessages(newLocale)
        i18n.global.setLocaleMessage(newLocale, messages)
      }

      // 2. 更新 Vue I18n 实例
      if (typeof i18n.global.locale === 'object' && 'value' in i18n.global.locale) {
        i18n.global.locale.value = newLocale
      } else {
        i18n.global.locale = newLocale
      }
      currentLocale.value = newLocale

      // 3. 保存到本地存储
      localStorage.setItem('app-locale', newLocale)

      // 4. 更新 HTML lang 属性
      document.documentElement.lang = newLocale

      // 5. 通知后端语言变更并持久化
      try {
        await invoke('set_app_locale_with_persistence', { locale: newLocale })
        console.log(`📝 语言已切换到: ${newLocale}`)
      } catch (error) {
        console.warn('Failed to notify backend of locale change:', error)
      }

    } catch (error) {
      console.error(`Failed to switch language to ${newLocale}:`, error)
    }
  }

  // 获取浏览器语言偏好
  const getBrowserLocale = (): SupportedLocale => {
    const browserLang = navigator.language

    // 直接匹配
    if (SUPPORTED_LOCALES.includes(browserLang as SupportedLocale)) {
      return browserLang as SupportedLocale
    }

    // 基础语言匹配 (如 'zh' 匹配 'zh-CN')
    const baseLang = browserLang.split('-')[0]
    const matchedLocale = SUPPORTED_LOCALES.find(locale => 
      locale.startsWith(baseLang)
    )

    return matchedLocale || 'zh-CN'
  };

  // 初始化语言设置
  const initializeLocale = async () => {
    try {
      // 1. 首先尝试从后端获取保存的语言设置
      const backendLocale = await invoke('get_app_locale') as SupportedLocale
      if (backendLocale && SUPPORTED_LOCALES.includes(backendLocale)) {      
        await setLocale(backendLocale)
        return
      }
    } catch (error) {
      console.warn('Failed to get locale from backend, using local settings:', error)
    }

    // 2. 回退到本地存储和浏览器设置
    const savedLocale = localStorage.getItem('app-locale') as SupportedLocale
    const targetLocale = savedLocale && SUPPORTED_LOCALES.includes(savedLocale) 
      ? savedLocale 
      : getBrowserLocale()
    
    await setLocale(targetLocale)
  }

  return {
    locale,
    availableLocales,
    currentLanguageInfo,
    setLocale,
    getBrowserLocale,
    initializeLocale
  }
}

/**
 * 格式化工具组合式函数
 */
export function useFormatting() {
  const { d, n } = useI18n()

  // 格式化文件大小
  const formatFileSize = (bytes: number): string => {
    const units = ['B', 'KB', 'MB', 'GB', 'TB']
    let size = bytes
    let unitIndex = 0

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }

    return `${size.toFixed(2)} ${units[unitIndex]}`
  }

  // 格式化相对时间
  const formatRelativeTime = (date: Date | string | number): string => {
    const targetDate = new Date(date)
    const now = new Date()
    const diffInSeconds = Math.floor((now.getTime() - targetDate.getTime()) / 1000)

    if (diffInSeconds < 60) return '刚刚'
    if (diffInSeconds < 3600) {
      const minutes = Math.floor(diffInSeconds / 60)
      return `${minutes} 分钟前`
    }
    if (diffInSeconds < 86400) {
      const hours = Math.floor(diffInSeconds / 3600)
      return `${hours} 小时前`
    }
    const days = Math.floor(diffInSeconds / 86400)
    return `${days} 天前`
  }

  // 格式化百分比
  const formatPercentage = (value: number, decimals: number = 1): string => {
    return `${(value * 100).toFixed(decimals)}%`
  }

  return {
    d,
    n,
    formatFileSize,
    formatRelativeTime,
    formatPercentage
  }
}

// 监听语言变化，同步到全局状态
watch(currentLocale, (newLocale) => {
  console.log('📝 语言设置已更新并持久化:', newLocale)
}, { immediate: true })