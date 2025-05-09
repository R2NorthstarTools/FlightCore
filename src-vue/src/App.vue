<script lang="ts">
import ChangelogView from './views/ChangelogView.vue';
import DeveloperView from './views/DeveloperView.vue';
import PlayView from './views/PlayView.vue';
import ModsView from './views/ModsView.vue';
import SettingsView from './views/SettingsView.vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { store } from './plugins/store';
import { load } from '@tauri-apps/plugin-store';
import { invoke } from "@tauri-apps/api/core";
import NotificationButton from "./components/NotificationButton.vue";

export default {
  components: {
      NotificationButton,
      ChangelogView,
      DeveloperView,
      PlayView,
      SettingsView,
      ModsView
  },
  data() {
    return {}
  },
  mounted: async function() {
    store.commit('initialize');

    // Initialize interface language
    const persistentStore = await load('flight-core-settings.json', { autoSave: false });
    let lang: string | null | undefined = await persistentStore.get('lang');
    if (lang === null || lang == undefined) {
      lang = navigator.language.substring(0, 2);
      persistentStore.set('lang', lang);
      await persistentStore.save();
    }
    this.$root!.$i18n.locale = lang;
  },
  methods: {
    async minimize() {
      await getCurrentWindow().minimize();
    },
    close() {
      invoke("close_application");
    }
  },
    computed: {
      bgStyle(): string {
          // @ts-ignore
          const shouldBlur = this.$route.path !== "/";
          return `filter: brightness(0.8) ${shouldBlur ? 'blur(5px)' : ''};`;
      }
    }
}
</script>

<template>
  <div class="app-inner">
    <div id="fc_bg__container" :style="bgStyle"/>

    <nav id="fc_menu-bar" v-if="$route.path !== '/repair'"><!-- Hide menu bar in repair view -->
      <!-- Navigation items -->
      <el-menu
        :default-active="$route.path"
        router
        mode="horizontal"
        id="fc__menu_items"
        data-tauri-drag-region
      >
        <el-menu-item index="/">{{ $t('menu.play') }}</el-menu-item>
        <el-menu-item index="/mods">{{ $t('menu.mods') }}</el-menu-item>
        <el-menu-item index="/changelog">{{ $t('menu.changelog') }}</el-menu-item>
        <el-menu-item index="/settings">{{ $t('menu.settings') }}</el-menu-item>
        <el-menu-item index="/dev" v-if="$store.state.developer_mode">{{ $t('menu.dev') }}</el-menu-item>
      </el-menu>

      <!-- Window controls -->
      <div id="fc_window__controls">
        <NotificationButton />
        <el-button color="white" icon="SemiSelect" @click="minimize" circle />
        <el-button color="white" icon="CloseBold" @click="close" circle />
      </div>
    </nav>

    <router-view></router-view>
  </div>
</template>

<style>
#fc_menu-bar {
  position: fixed;
  z-index: 1;
  top: 0;
  width: 100%;
  height: var(--fc-menu_height);
  /* Colour header bar to keep text readable */
  background-color: rgba(0, 0, 0, 0.7);
}

#fc__menu_bar::before {
    position: absolute;
    content: "";
    inset: 0; /* same as { top: 0; right: 0; bottom: 0; left: 0; } */
    background-image: linear-gradient(to bottom, red, orange);
    z-index: 1;
    opacity: 0;
    transition: opacity 1s linear;
}

#fc__menu_bar:hover::before {
    opacity: 1;
}

/* Borders reset */
#fc__menu_bar, #fc__menu_items {
    border: none !important;
}
.app-inner {
  height: 100%;
  width: 100%;
}

/* Header item */
#fc__menu_items {
  height: 100%;
  background-color: transparent;
  float: left;
  width: calc(100% - 168px); /* window controls container width */
}

#fc__menu_items .el-menu-item, #fc__menu_items .el-sub-menu__title {
  color: #b4b6b9;
  border-color: white;
}

#fc__menu_items > .el-menu-item {
  text-transform: uppercase;
  border: none !important;
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
  font-weight: bold;
  font-size: large;
  background-color: transparent !important;

  border-width: 2px !important;
  border-style: solid !important;
  border-color: transparent !important;
  border-radius: 10px !important;
  transition: none;
}

#fc__menu_items .el-menu-item:hover, #fc__menu_items .el-sub-menu__title {
  color: #c6c9ce;
  background-color: transparent;
}

#fc__menu_items .el-menu-item:focus-visible {
  border-color: rgb(160, 207, 255) !important;
}

#fc__menu_items .el-menu-item.is-active, #fc__menu_items .el-sub-menu.is-active > .el-sub-menu__title {
  color: white !important;
}

.app-inner > .fc__mods__container {
  overflow-y: auto;
  height: calc(100% - var(--fc-menu_height));
}

/* Header menu */
.developer_build {
  background: repeating-linear-gradient(
    45deg,
    rgba(0, 0, 0, 0.2),
    rgba(0, 0, 0, 0.2) 20px,
    rgba(0, 0, 0, 0.3) 20px,
    rgba(0, 0, 0, 0.3) 40px
  );
}

/* Window controls */
#fc_window__controls {
  float: right;
  height: 100%;
}

#fc_window__controls > button,
#fc_window__controls > .el-dropdown > button,
#fc_window__controls > .el-dropdown > .el-badge > button {
  color: white;
  font-size: 20px;
  margin: auto 5px;
  background: none;
  border: none;
  height: 100%;
}

#fc_window__controls > button:hover,
#fc_window__controls > .el-dropdown > button:hover,
#fc_window__controls > .el-dropdown > .el-badge > button:hover {
  color: #c6c9ce;
}

#fc_window__controls > button:active,
#fc_window__controls > .el-dropdown > button:active {
  color: #56585a;
}

#fc_window__controls > button:last-of-type {
  margin-right: 15px;
}

sup {
  border: none !important;
}

</style>
