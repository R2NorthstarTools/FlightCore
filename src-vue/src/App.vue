<script setup lang="ts">
</script>

<template>
  <el-tabs class="fc_menu__tabs" type="card">
    <el-tab-pane label="Play">Play</el-tab-pane>
    <el-tab-pane label="Changelog">Changelog</el-tab-pane>
    <el-tab-pane label="Mods">Mods</el-tab-pane>
    <el-tab-pane label="Settings">Settings</el-tab-pane>
  </el-tabs>
</template>

<style>
/* Borders reset */
.fc_menu__tabs .el-tabs__nav, .fc_menu__tabs .el-tabs__header {
  border: none !important;
}

/* Header item */
.fc_menu__tabs .el-tabs__item {
  color: #898b8e;
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
  background-color: #3c4549;
  height: auto !important;
}

</style>
