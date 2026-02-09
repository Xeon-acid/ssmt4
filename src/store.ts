import { reactive, watch } from 'vue'

// Define the shape of our settings
interface AppSettings {
  bgType: 'image' | 'video';
  bgImage: string;
  bgVideo: string;
  sidebarOpacity: number;
  sidebarBlur: number;
  contentOpacity: number;
  contentBlur: number;
}

// Load from localStorage or use defaults
const saved = localStorage.getItem('ssmt4-settings')
const defaultSettings: AppSettings = {
  bgType: 'image',
  bgImage: '/background.png',
  bgVideo: '/background.webm',
  sidebarOpacity: 0.3, // Lower default for dark theme transparency
  sidebarBlur: 20,
  contentOpacity: 0.2, // Lower default for dark theme transparency
  contentBlur: 3
}

export const appSettings = reactive<AppSettings>(
  saved ? { ...defaultSettings, ...JSON.parse(saved) } : defaultSettings
)

// Auto-save behavior
watch(appSettings, (newVal) => {
  localStorage.setItem('ssmt4-settings', JSON.stringify(newVal))
})
