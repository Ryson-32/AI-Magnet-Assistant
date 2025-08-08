import common from './common.json'
import homeMessages from './pages/home.json'
import settingsMessages from './pages/settings.json'
import favoritesMessages from './pages/favorites.json'
import enginesMessages from './pages/engines.json'
import priorityMessages from './pages/priority.json'
import errorMessages from './messages/errors.json'
import resultCardMessages from './components/resultcard.json'

// 简单但安全的深度合并函数
function mergeDeep(target: any, source: any): any {
  for (const key in source) {
    if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
      if (!target[key] || typeof target[key] !== 'object') {
        target[key] = {}
      }
      mergeDeep(target[key], source[key])
    } else {
      target[key] = source[key]
    }
  }
  return target
}

// 正确的深度合并，保持嵌套结构
const messages = mergeDeep({}, common)
mergeDeep(messages, homeMessages)
mergeDeep(messages, settingsMessages)
mergeDeep(messages, favoritesMessages)
mergeDeep(messages, enginesMessages)
mergeDeep(messages, priorityMessages)
mergeDeep(messages, errorMessages)

// 手动添加组件翻译
if (!messages.components) {
  messages.components = {}
}
mergeDeep(messages.components, resultCardMessages.components)

// 添加导航组件翻译
messages.components.navigation = {
  title: "AI Magnet Assistant",
  menu: "Menu",
  toggleSidebar: "Toggle Sidebar"
}

// 添加语言切换器翻译
messages.components.languageSwitcher = {
  selectLanguage: "Select Language",
  currentLanguage: "Current Language"
}

export default messages;