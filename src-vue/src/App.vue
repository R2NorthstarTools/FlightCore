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
    document.querySelector(".el-tabs__nav-scroll")!.addEventListener("mousedown", async e => {
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
    <div id="fc_bg__container" />
    <el-tabs v-model="$store.state.current_tab" class="fc_menu__tabs" type="card">
        <el-tab-pane name="Play" label="Play"><PlayView /></el-tab-pane>
        <el-tab-pane name="Changelog" label="Changelog"><ChangelogView /></el-tab-pane>
        <!-- <el-tab-pane label="Mods">Mods</el-tab-pane> -->
        <el-tab-pane name="Settings" label="Settings"><SettingsView/></el-tab-pane>
        <el-tab-pane v-if="$store.state.developer_mode" name="Dev" label="Dev">
          <DeveloperView/>
        </el-tab-pane>
    </el-tabs>
    <div id="fc_window__controls">
        <el-button color="white" icon="SemiSelect" @click="minimize" circle />
        <el-button color="white" icon="CloseBold" @click="close" circle />
    </div>
</template>

<style>
/* Borders reset */
.fc_menu__tabs .el-tabs__nav, .fc_menu__tabs .el-tabs__header {
  border: none !important;
}

/* Header item */
.fc_menu__tabs .el-tabs__item {
  color: #b4b6b9;
  text-transform: uppercase;
  border: none !important;
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', '微软雅黑', Arial, sans-serif;
  font-weight: bold;
  font-size: large;
  margin: 10px 0;
}

.fc_menu__tabs .el-tabs__item:hover {
  color: #c6c9ce;
}

.fc_menu__tabs .is-active {
  color: white !important;
}

/* Header menu */
.fc_menu__tabs .el-tabs__header {
  background-image: radial-gradient(transparent 1px);
  backdrop-filter: saturate(50%) blur(4px);
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
