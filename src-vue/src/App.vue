<script lang="ts">
import ChangelogView from './views/ChangelogView.vue';
import DeveloperView from './views/DeveloperView.vue';
import PlayView from './views/PlayView.vue';
import SettingsView from './views/SettingsView.vue';
import { appWindow } from '@tauri-apps/api/window';
import { store } from './plugins/store';
import { window as tauriWindow } from "@tauri-apps/api";

export default {
  components: {
      ChangelogView,
      DeveloperView,
      PlayView,
      SettingsView
  },
  data() {
    return {}
  },
  mounted: () => {
    store.commit('initialize');

    // Enable dragging entire app by dragging menu bar.
    // https://github.com/tauri-apps/tauri/issues/1656#issuecomment-1161495124
    document.querySelector("#fc__menu_bar")!.addEventListener("mousedown", async e => {
        if ((e.target as Element).closest(".el-menu-item")) return; // Disable drag when clicking menu items.
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
    <div id="fc_bg__container" />

    <el-menu
        default-active="/"
        router
        mode="horizontal"
        id="fc__menu_bar"
    >
        <el-menu-item active index="/">Play</el-menu-item>
        <el-menu-item index="/changelog">Changelog</el-menu-item>
        <el-menu-item index="/settings">Settings</el-menu-item>
        <el-menu-item index="/dev" v-if="$store.state.developer_mode">Dev</el-menu-item>
    </el-menu>

    <router-view></router-view>

    <div id="fc_window__controls">
        <el-button color="white" icon="SemiSelect" @click="minimize" circle />
        <el-button color="white" icon="CloseBold" @click="close" circle />
    </div>
</template>

<style>
/* Borders reset */
#fc__menu_bar {
    border: none !important;
}

/* Header item */
#fc__menu_bar .el-menu-item {
  color: #b4b6b9;
  text-transform: uppercase;
  border: none !important;
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
  font-weight: bold;
  font-size: large;
}

#fc__menu_bar .el-menu-item:hover {
  color: #c6c9ce;
  background-color: transparent;
}

#fc__menu_bar .el-menu-item.is-active {
  color: white !important;
  background-color: transparent;
}

/* Header menu */
#fc__menu_bar {
  background-image: radial-gradient(transparent 1px);
  backdrop-filter: saturate(50%) blur(4px);
  background-color: transparent;
  height: auto !important;
}

/* Window controls */
#fc_window__controls {
  display: flex;
  position: absolute;
  top: 0;
  right: 0;
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

#fc_window__controls > button:last-of-type {
  margin-right: 15px;
}

</style>
