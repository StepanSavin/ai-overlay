import { createPinia } from 'pinia';
import { createApp } from 'vue';

import App from './app/app.vue';
import './app/styles/index.css';

createApp(App).use(createPinia()).mount('#app');
