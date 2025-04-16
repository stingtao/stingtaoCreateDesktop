import { createI18n } from 'vue-i18n'
import en from '../locales/en.json'
import zhTW from '../locales/zh_TW.json'
import ja from '../locales/ja.json'

const messages = {
  'en': en,
  'zh_TW': zhTW,
  'ja': ja
}

// 從 localStorage 加載保存的語言設置，如果沒有則默認為英文
const savedLocale = localStorage.getItem('locale') || 'en'

export const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'en',
  messages
})

export const availableLocales = [
  { code: 'en', name: 'English' },
  { code: 'zh_TW', name: '繁體中文' },
  { code: 'ja', name: '日本語' }
] 