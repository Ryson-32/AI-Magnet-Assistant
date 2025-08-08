import { createI18n, type I18n } from 'vue-i18n'

// 支持的语言列表
export const SUPPORTED_LOCALES = ['zh-CN', 'en'] as const;

// 默认语言
export const DEFAULT_LOCALE = 'zh-CN';

// 备用语言
export const FALLBACK_LOCALE = 'en';

// 语言类型
export type SupportedLocale = typeof SUPPORTED_LOCALES[number];

// 检测用户语言偏好
export function detectUserLocale(): SupportedLocale {
  // 1. 从 localStorage 获取保存的语言偏好
  const savedLocale = localStorage.getItem('app-locale') as SupportedLocale
  if (savedLocale && SUPPORTED_LOCALES.includes(savedLocale)) {
    return savedLocale;
  }

  // 2. 从浏览器语言设置获取
  const browserLocale = navigator.language
  if (SUPPORTED_LOCALES.includes(browserLocale as SupportedLocale)) {
    return browserLocale as SupportedLocale
  }

  // 3. 检查是否有相近的语言匹配
  const normalizedBrowserLocale = browserLocale.split('-')[0]
  for (const locale of SUPPORTED_LOCALES) {
    if (locale.startsWith(normalizedBrowserLocale)) {
      return locale;
    }
  }

  // 4. 返回默认语言
  return DEFAULT_LOCALE;
}

// 动态加载语言包
export async function loadLocaleMessages(locale: SupportedLocale) {

  try {
    const messages = await import(`./locales/${locale}/index.ts`)
    return messages.default
  } catch (error) {
    console.warn(`Failed to load locale messages for ${locale}:`, error)
    // 回退到英文
    if (locale !== FALLBACK_LOCALE) {
      return await loadLocaleMessages(FALLBACK_LOCALE)
    }
    return {}
  }
}

// 创建 i18n 实例。
export async function createI18nInstance() {
  const locale = detectUserLocale();

  // 加载所有支持的语言包以避免动态加载问题
  const allMessages: Record<string, any> = {};

  for (const supportedLocale of SUPPORTED_LOCALES) {
    try {
      const localeMessages = await loadLocaleMessages(supportedLocale);
      allMessages[supportedLocale] = localeMessages;
    } catch (error) {
      console.warn(`Failed to preload locale ${supportedLocale}:`, error)
    }
  }

  const i18n = createI18n({
    legacy: false,  // 使用Composition API模式，解决legacy模式错误
    locale,
    fallbackLocale: FALLBACK_LOCALE,
    messages: allMessages, // 使用预加载的所有语言包
    globalInjection: true, // 全局注入 $t
    missingWarn: false, // 关闭缺失翻译警告 - 我们已实现完整的备选访问方案
    fallbackWarn: false, // 关闭回退警告 - 减少开发环境噪音
    formatFallbackMessages: true, // 格式化回退消息
    // 添加语言映射以防止zh-CN被简化为zh
    datetimeFormats: {
      'zh-CN':  {},
      'en': {}
    },
    numberFormats: {
      'zh-CN': {},
      'en': {} 
    }
  })

  return i18n
}

// 单例 i18n 实例（延迟初始化）
let i18nInstance: I18n | null = null

// 获取 i18n 实例
export function getI18n() {
  if (!i18nInstance) {
    throw new Error('i18n instance not initialized. Call setupI18n() first.')
  }
  return i18nInstance
}

// 设置 i18n 实例
export async function setupI18n() {
  if (!i18nInstance) {
    i18nInstance = await createI18nInstance()
  }
  return i18nInstance
}
