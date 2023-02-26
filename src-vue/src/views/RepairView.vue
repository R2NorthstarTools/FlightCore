<template>
    <div class="fc-container">
        <el-scrollbar>
            <el-alert title="Info" type="info" :closable="false" show-icon>
                This window contains various functionality to repair common issues with Northstar and FlightCore.
            </el-alert>

            <h1>Repair</h1>

            <h2>Northstar</h2>

            <el-button type="primary" @click="disableAllModsButCore">
                Disable all but core mods
            </el-button>

        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { GameInstall } from "../utils/GameInstall";
import { invoke } from "@tauri-apps/api";

export default defineComponent({
    name: "RepairView",
    methods: {
        async disableAllModsButCore() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("disable_all_but_core", { gameInstall: game_install })
                .then((message) => {
                    ElNotification({
                        title: 'Success',
                        message: "Disabled all mods but core",
                        type: 'success',
                        position: 'bottom-right'
                    });
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                });
        },
    }
});
</script>

<style scoped>
.fc-container {
    padding-top: 0px;
}
</style>
