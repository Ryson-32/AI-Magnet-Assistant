// 语言相关类型
export type SupportedLocale = 'zh-CN' | 'en'

// 语言信息接口
export interface LanguageInfo {
  code: SupportedLocale
  name: string
  nativeName: string
  flag: string
}

// 可用语言列表
export const AVAILABLE_LANGUAGES: LanguageInfo[] = [
  { code: 'zh-CN', name: 'Chinese', nativeName: '简体中文', flag: '🇨🇳' },
  { code: 'en', name: 'English', nativeName: 'English', flag: '🇺🇸' }
]

// 消息结构的类型定义
export interface MessageSchema {
  common: {
    navigation: {
      home: string
      favorites: string
      engines: string  
      settings: string
    }
    actions: {
      add: string
      remove: string
      save: string
      cancel: string
      search: string
    }
  }
  components: {
    resultCard: {
      priority: string
      actions: {
        goToSource: string
        addToFavorites: string
        copyMagnetLink: string
        copied: string
        quickDownload: string
        opening: string
      }
      analysis: {
        score: string
      }
      fileList: {
        moreFiles: string
        expandHint: string
        collapseHint: string
      }
      messages: {
        downloadStarted: string
        downloadFailed: string
      }
    }
  }
  pages: {
    home: {
      title: string
      subtitle: string
      search: {
        placeholder: string
        advancedSearchToggle: string
        searchButton: string
      }
      advancedSearch: {
        title: string
        engines: string
        fileSize: {
          label: string
          min: string
          max: string
          placeholder: string
        }
        fileType: {
          label: string
          placeholder: string
        }
        sortBy: {
          label: string
          relevance: string
          size: string
          seeders: string
          date: string
        }
      }
      filters: {
        clearAll: string
        activeFilters: string
      }
      results: {
        found: string
        noResults: string
        loading: string
        searchToStart: string
      }
    }
    settings: {
      title: string
      subtitle: string
      language: {
        title: string
        description: string
        current: string
        switch: string
        switchSuccess: string
        switchError: string
      }
      other: {
        title: string
        description: string
      }
    }
    engines: {
      title: string
      subtitle: string
      list: {
        title: string
        empty: string
        emptyMessage: string
        loading: string
      }
      table: {
        name: string
        status: string
        actions: string
      }
      status: {
        enabled: string
        disabled: string
      }
      actions: {
        enable: string
        disable: string
        delete: string
      }
      add: {
        title: string
        name: {
          label: string
          placeholder: string
        }
        baseUrl: {
          label: string
          placeholder: string
        }
        searchPath: {
          label: string
          placeholder: string
        }
        cancel: string
        save: string
      }
      messages: {
        loadFailed: string
        enableSuccess: string
        enableFailed: string
        disableSuccess: string
        disableFailed: string
        deleteSuccess: string
        deleteFailed: string
        addSuccess: string
        addFailed: string
        validation: {
          nameRequired: string
          baseUrlRequired: string
          searchPathRequired: string
          invalidUrl: string
        }
      }
    }
    favorites: {
      title: string
      subtitle: string
      search: {
        placeholder: string
        refresh: string
      }
      list: {
        title: string
        loading: string
        empty: string
        emptyMessage: string
      }
      item: {
        fileSize: string
        created: string
        files: string
        filesCount: string
        moreFiles: string
        actions: {
          copy: string
          remove: string
          confirmRemove: string
        }
      }
      messages: {
        loadFailed: string
        removeFailed: string
        copied: string
        copyFailed: string
      }
    }
    priority: {
      title: string
      subtitle: string
      add: {
        title: string
        placeholder: string
        addButton: string
        adding: string
      }
      list: {
        title: string
        count: string
        loading: string
        empty: string
        emptyMessage: string
      }
      item: {
        badge: string
        deleteTitle: string
        confirmDeleteTitle: string
      }
      info: {
        howItWorks: {
          title: string
          autoPriority: string
          visualHighlight: string
          caseInsensitive: string
          partialMatching: string
        }
        tips: {
          title: string
          specific: string
          trustedGroups: string
          qualityIndicators: string
          avoidCommon: string
        }
      }
      messages: {
        loadFailed: string
        addFailed: string
        deleteFailed: string
        validation: {
          enterKeyword: string
          keywordExists: string
        }
      }
    }
  }
}

// 翻译键类型 - 基于 MessageSchema 结构的嵌套键路径
export type TranslationKey = 
  | `common.navigation.${keyof MessageSchema['common']['navigation']}`
  | `common.actions.${keyof MessageSchema['common']['actions']}`
  | `components.resultCard.${keyof MessageSchema['components']['resultCard']}`
  | `components.resultCard.actions.${keyof MessageSchema['components']['resultCard']['actions']}`
  | `components.resultCard.analysis.${keyof MessageSchema['components']['resultCard']['analysis']}`
  | `components.resultCard.fileList.${keyof MessageSchema['components']['resultCard']['fileList']}`
  | `components.resultCard.messages.${keyof MessageSchema['components']['resultCard']['messages']}`
  | `pages.home.${keyof MessageSchema['pages']['home']}`
  | `pages.home.search.${keyof MessageSchema['pages']['home']['search']}`
  | `pages.home.advancedSearch.${keyof MessageSchema['pages']['home']['advancedSearch']}`
  | `pages.home.advancedSearch.fileSize.${keyof MessageSchema['pages']['home']['advancedSearch']['fileSize']}`
  | `pages.home.advancedSearch.fileType.${keyof MessageSchema['pages']['home']['advancedSearch']['fileType']}`
  | `pages.home.advancedSearch.sortBy.${keyof MessageSchema['pages']['home']['advancedSearch']['sortBy']}`
  | `pages.home.filters.${keyof MessageSchema['pages']['home']['filters']}`
  | `pages.home.results.${keyof MessageSchema['pages']['home']['results']}`
  | `pages.settings.${keyof MessageSchema['pages']['settings']}`
  | `pages.settings.language.${keyof MessageSchema['pages']['settings']['language']}`
  | `pages.settings.other.${keyof MessageSchema['pages']['settings']['other']}`
  | `pages.engines.${keyof MessageSchema['pages']['engines']}`
  | `pages.engines.list.${keyof MessageSchema['pages']['engines']['list']}`
  | `pages.engines.table.${keyof MessageSchema['pages']['engines']['table']}`
  | `pages.engines.status.${keyof MessageSchema['pages']['engines']['status']}`
  | `pages.engines.actions.${keyof MessageSchema['pages']['engines']['actions']}`
  | `pages.engines.add.${keyof MessageSchema['pages']['engines']['add']}`
  | `pages.engines.add.name.${keyof MessageSchema['pages']['engines']['add']['name']}`
  | `pages.engines.add.baseUrl.${keyof MessageSchema['pages']['engines']['add']['baseUrl']}`
  | `pages.engines.add.searchPath.${keyof MessageSchema['pages']['engines']['add']['searchPath']}`
  | `pages.engines.messages.${keyof MessageSchema['pages']['engines']['messages']}`
  | `pages.engines.messages.validation.${keyof MessageSchema['pages']['engines']['messages']['validation']}`
  | `pages.favorites.${keyof MessageSchema['pages']['favorites']}`
  | `pages.favorites.search.${keyof MessageSchema['pages']['favorites']['search']}`
  | `pages.favorites.list.${keyof MessageSchema['pages']['favorites']['list']}`
  | `pages.favorites.item.${keyof MessageSchema['pages']['favorites']['item']}`
  | `pages.favorites.item.actions.${keyof MessageSchema['pages']['favorites']['item']['actions']}`
  | `pages.favorites.messages.${keyof MessageSchema['pages']['favorites']['messages']}`
  | `pages.priority.${keyof MessageSchema['pages']['priority']}`
  | `pages.priority.add.${keyof MessageSchema['pages']['priority']['add']}`
  | `pages.priority.list.${keyof MessageSchema['pages']['priority']['list']}`
  | `pages.priority.item.${keyof MessageSchema['pages']['priority']['item']}`
  | `pages.priority.info.howItWorks.${keyof MessageSchema['pages']['priority']['info']['howItWorks']}`
  | `pages.priority.info.tips.${keyof MessageSchema['pages']['priority']['info']['tips']}`
  | `pages.priority.messages.${keyof MessageSchema['pages']['priority']['messages']}`
  | `pages.priority.messages.validation.${keyof MessageSchema['pages']['priority']['messages']['validation']}`
  | string  // 允许其他字符串以保持灵活性

// 翻译参数类型
export type TranslationParams = Record<string, string | number | boolean>
