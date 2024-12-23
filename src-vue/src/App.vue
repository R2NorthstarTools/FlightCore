<script lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

export default {
  components: {
  },
  data() {
    return {}
  },
  mounted: async function() {

    // Initialize interface language
  },
  methods: {
    close() {
      invoke("close_application");
    }
  },
    computed: {
    }
}
</script>

<template>
  <div class="app-inner">
    <div id="fc_bg__container" :style="bgStyle"/>

    <nav id="fc_menu-bar"><!-- Hide menu bar in repair view -->
      <!-- Navigation items -->
    </nav>
    <div class="row">
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </div>
</template>

<style>
#fc_menu-bar {
  position: fixed;
  z-index: 1;
  top: 0;
  width: 100%;
  height: var(--fc-menu_height);
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
