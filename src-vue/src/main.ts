import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from "element-plus";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import { store } from './plugins/store';
import PlayView from "./views/PlayView.vue";
import ChangelogView from "./views/ChangelogView.vue";
import ModsView from "./views/ModsView.vue";
import SettingsView from "./views/SettingsView.vue";
import DeveloperView from "./views/DeveloperView.vue";
import {createRouter, createWebHashHistory} from "vue-router";


const app = createApp(App);


// styles
import 'element-plus/theme-chalk/index.css';
import './style.css'
import ThunderstoreModsView from "./views/ThunderstoreModsView.vue";

app.use(ElementPlus);

// icons
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

// style
app.use( store, '$store' );


// routes
const routes = [
    { path: '/', name: 'Main', component: async () => PlayView},
    { path: '/changelog', name: 'Changelog', component: async () => ChangelogView},
    { path: '/mods', name: 'Mods', component: async () => ModsView},
    { path: '/thunderstoreMods', name: 'Thunderstore mods', component: async () => ThunderstoreModsView},
    { path: '/settings', name: 'Settings', component: async () => SettingsView},
    { path: '/dev', name: 'Dev', component: async () => DeveloperView}
];
export const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
});
app.use(router);


app.mount('#app')
