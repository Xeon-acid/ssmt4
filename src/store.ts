import { reactive, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { message } from '@tauri-apps/plugin-dialog'

// Define the shape of our settings
export interface AppSettings {
  bgType: 'image' | 'video';
  bgImage: string;
  bgVideo: string;
  sidebarOpacity: number;
  sidebarBlur: number;
  contentOpacity: number;
  contentBlur: number;
  cacheDir: string;
}

const defaultSettings: AppSettings = {
  bgType: 'image',
  bgImage: '/background.png',
  bgVideo: '/background.webm',
  sidebarOpacity: 0.3, // Lower default for dark theme transparency
  sidebarBlur: 20,
  contentOpacity: 0.2, // Lower default for dark theme transparency
  contentBlur: 3,
  cacheDir: ''
}

export const appSettings = reactive<AppSettings>({ ...defaultSettings })

// Load from backend - replaced below


// Initial load
let isInitialized = false;

async function loadSettings() {
  try {
    const loaded = await invoke<AppSettings>('load_settings')
    console.log('Loaded settings from backend:', loaded);
    
    // Update reactive object with loaded values
    Object.assign(appSettings, loaded)
    
    // Mark as initialized so watch can start saving changes
    // using setTimeout to ensure the current tick doesn't trigger watch
    setTimeout(() => {
        isInitialized = true;
    }, 100);
    
  } catch (e) {
    console.error('Failed to load settings:', e)
    await message(`加载设置失败: ${e}`, { title: '错误', kind: 'error' });
  }
}

// Initial load
loadSettings()

// Auto-save behavior
watch(appSettings, async (newVal) => {
  if (!isInitialized) {
      console.log('Skipping save because store is not yet initialized');
      return;
  }
  console.log('Saving settings:', newVal);
  try {
    await invoke('save_settings', { config: newVal })
  } catch (e) {
    console.error('Failed to save settings:', e)
  }
}, { deep: true })
