<script lang="ts">
import ChangelogView from './views/ChangelogView.vue';
import DeveloperView from './views/DeveloperView.vue';
import PlayView from './views/PlayView.vue';
import ModsView from './views/ModsView.vue';
import SettingsView from './views/SettingsView.vue';
import { appWindow } from '@tauri-apps/api/window';
import { store } from './plugins/store';
import { window as tauriWindow } from "@tauri-apps/api";

export default {
  components: {
      ChangelogView,
      DeveloperView,
      PlayView,
      SettingsView,
      ModsView
  },
  data() {
    return {}
  },
  mounted: () => {
    store.commit('initialize');
  },
  methods: {
    async toggleMaximize() {
      await appWindow.toggleMaximize();
    },
    minimize() {
      appWindow.minimize()
    },
    close() {
      appWindow.close()
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

    <nav id="fc_menu-bar">
      <!-- Navigation items -->
      <el-menu
        default-active="/"
        router
        mode="horizontal"
        id="fc__menu_items"
        data-tauri-drag-region
      >
        <el-menu-item index="/">Play</el-menu-item>
        <el-menu-item index="/changelog">Changelog</el-menu-item>
        <el-menu-item index="/mods">Mods</el-menu-item>
        <el-menu-item index="/thunderstoreMods">Thunderstore</el-menu-item>
        <el-menu-item index="/settings">Settings</el-menu-item>
        <el-menu-item index="/repair" v-if="$store.state.repair_view_visible">Repair</el-menu-item>
        <el-menu-item index="/dev" v-if="$store.state.developer_mode">Dev</el-menu-item>
      </el-menu>

      <!-- Window controls -->
      <div id="fc_window__controls">
        <el-button color="white" icon="SemiSelect" @click="minimize" circle />
        <el-button color="white" icon="FullScreen" @click="toggleMaximize" circle />
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
  background-image: radial-gradient(transparent 1px);
  backdrop-filter: saturate(50%) blur(4px);
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
  width: calc(100% - 148px); /* window controls container width */
}

#fc__menu_items .el-menu-item, #fc__menu_items .el-sub-menu__title {
  color: #b4b6b9;
  border-color: white;
}

.el-menu > .el-menu-item {
  text-transform: uppercase;
  border: none !important;
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
  font-weight: bold;
  font-size: large;
  background-color: transparent !important;
}

#fc__menu_items .el-menu-item:hover, #fc__menu_items .el-sub-menu__title {
  color: #c6c9ce;
  background-color: transparent;
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

#fc_window__controls > button {
  color: white;
  font-size: 20px;
  margin: auto 5px;
  background: none;
  border: none;
  height: 100%;
}

#fc_window__controls > button:hover {
  color: #c6c9ce;
}

#fc_window__controls > button:active {
  color: #56585a;
}

#fc_window__controls > button:last-of-type {
  margin-right: 15px;
}

</style>
