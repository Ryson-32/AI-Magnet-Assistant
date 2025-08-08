import { createApp } from "vue";
import App from "./App.vue";
import { setupI18n } from "./i18n";

async function bootstrap() {
  // 1. 创建 Vue 应用实例
  const app = createApp(App);

  try {
    // 2. 初始化 i18n 实例
    console.log('Initializing i18n...');
    const i18n = await setupI18n();
    
    // 3. 安装 i18n 插件
    app.use(i18n);
    
    console.log('i18n initialized successfully');
    const currentLocale = typeof i18n.global.locale === 'object' && 'value' in i18n.global.locale
      ? i18n.global.locale.value
      : i18n.global.locale;
    console.log('Current locale:', currentLocale);
    
    // 4. 挂载应用
    app.mount("#app");
    console.log('Vue application mounted successfully');
    
  } catch (error) {
    console.error('Failed to initialize application:', error);
    
    // 如果 i18n 初始化失败，仍然挂载应用（降级处理）
    console.log('Mounting application without i18n...');
    app.mount("#app");
  }
}

// 启动应用
bootstrap();
