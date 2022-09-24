import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from "element-plus";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'


// styles
import 'element-plus/theme-chalk/index.css';
import './style.css'
import { store } from './plugins/store';

const app = createApp(App);
app.use(ElementPlus);

// icons
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

// style
app.use( store );

app.mount('#app')
