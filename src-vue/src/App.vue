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

    // Enable dragging entire app by dragging menu bar.
    // https://github.com/tauri-apps/tauri/issues/1656#issuecomment-1161495124
    document.querySelector(".el-tabs__header")!.addEventListener("mousedown", async e => {
        if ((e.target as Element).closest(".el-tabs__item")) return; // Disable drag when clicking menu items.
        await tauriWindow.appWindow.startDragging();
    });
  },
  methods: {
    minimize() {
      appWindow.minimize()
    },
    close() {
      appWindow.close()
    }
  }
}
</script>

<template>
  <div class="app-inner">
    <div id="fc_bg__container" />
    <el-tabs v-model="$store.state.current_tab" class="fc_menu__tabs" type="card">
        <el-tab-pane name="Play" label="Play"><PlayView /></el-tab-pane>
        <el-tab-pane name="Changelog" label="Changelog"><ChangelogView /></el-tab-pane>
        <el-tab-pane name="Mods" label="Mods"><ModsView /></el-tab-pane>
        <el-tab-pane name="Settings" label="Settings"><SettingsView/></el-tab-pane>
        <el-tab-pane v-if="$store.state.developer_mode" name="Dev" label="Dev">
          <DeveloperView/>
        </el-tab-pane>
    </el-tabs>
    <div id="fc_window__controls">
        <el-button color="white" icon="SemiSelect" @click="minimize" circle />
        <el-button color="white" icon="CloseBold" @click="close" circle />
    </div>
  </div>
</template>

<style>
.app-inner {
  height: 100%;
  width: 100%;
  overflow: hidden;
  position: relative;
}

.app-inner > .fc_menu__tabs {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}

.app-inner > .fc_menu__tabs > .el-tabs__header {
  flex: 0 0 auto;

  background-image: radial-gradient(transparent 1px);
  backdrop-filter: saturate(50%) blur(4px);
  height: auto;
  margin: 0;
  padding: 10px 0;
  border: none;
}

.app-inner > .fc_menu__tabs > .el-tabs__header .el-tabs__nav {
  border: none !important;
}

.app-inner > .fc_menu__tabs > .el-tabs__header .el-tabs__item {
  color: #b4b6b9;
  text-transform: uppercase;
  border: none;
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
  font-weight: bold;
  font-size: large;
}

.app-inner > .fc_menu__tabs > .el-tabs__header .el-tabs__item:hover {
  color: #c6c9ce;
}

.app-inner > .fc_menu__tabs > .el-tabs__header .is-active {
  color: white !important;
}

.app-inner > .fc_menu__tabs > .el-tabs__content {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
}

#fc_bg__container {
  background: url(/src/assets/wallpaperflare.com_wallpaper.jpg) center no-repeat;
  background-size: cover;
  height: 100%;
  width: 100%;
  position: fixed;
  filter: brightness(0.8);
}

/* Window controls */
#fc_window__controls {
  display: flex;
  position: absolute;
  top: 0;
  right: 15px;
  height: var(--el-tabs-header-height);
}

#fc_window__controls > button {
  color: white;
  font-size: 20px;
  margin: auto 5px;
  background: none;
  border: none;
}

#fc_window__controls > button:hover {
  color: #c6c9ce;
}

#fc_window__controls > button:active {
  color: #56585a;
}
</style>
