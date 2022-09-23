<script setup lang="ts">
import PlayView from './views/PlayView.vue';
import { appWindow } from '@tauri-apps/api/window';
</script>

<template>
  <div id="fc_bg__container" data-tauri-drag-region />
  <el-tabs class="fc_menu__tabs" type="card">
    <el-tab-pane label="Play"><PlayView /></el-tab-pane>
    <el-tab-pane label="Changelog">Changelog</el-tab-pane>
    <!-- <el-tab-pane label="Mods">Mods</el-tab-pane> -->
    <el-tab-pane label="Settings">Settings</el-tab-pane>
  </el-tabs>
  <div id="fc_window__controls">
    <el-button color="white" icon="SemiSelect" @click="appWindow.minimize()" circle />
    <el-button color="white" icon="CloseBold" @click="appWindow.close()" circle />
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
