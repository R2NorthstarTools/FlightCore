import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from "element-plus";

// styles
import 'element-plus/theme-chalk/index.css';
import './style.css'

const app = createApp(App);
app.use(ElementPlus);
app.mount('#app')
