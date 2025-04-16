import { createApp } from "vue";
import App from "./App.vue";
import { i18n } from "./lib/i18n";
import router from "./router";
import { invoke } from '@tauri-apps/api/tauri';
import { initDatabase } from './lib/sqlite';
import './assets/theme.css';

const initApp = async () => {
  try {
    // Initialize Tauri
    if (typeof window.__TAURI__ !== 'undefined') {
      console.log('Tauri is available, initializing database...');
      await initDatabase();
    } else {
      console.warn('Tauri is not available, running in web mode');
    }

    const app = createApp(App);
    app.use(i18n);
    app.use(router);
    app.mount("#app");
  } catch (error) {
    console.error('Failed to initialize app:', error);
  }
};

initApp();
