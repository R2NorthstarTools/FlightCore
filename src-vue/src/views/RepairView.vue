<template>
    <div class="fc-container">
        <el-scrollbar>
            <h2>Repair:</h2>

            <h3>Northstar:</h3>

            <el-button type="primary" @click="disableAllModsButCore">
                Disable all but core mods
            </el-button>

            <h3>FlightCore:</h3>

            <el-button type="primary" @click="cleanUpDownloadFolder">
                Force delete temp download folder
            </el-button>

            <el-button type="primary" @click="clearFlightCorePersistentStore">
                Delete FlightCore persistent store
            </el-button>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";
import { GameInstall } from "../utils/GameInstall";
import { Store } from 'tauri-plugin-store-api';
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: "Repair",
    methods: {
        async disableAllModsButCore() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("disable_all_but_core_caller", { gameInstall: game_install }).then((message) => {
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
        async cleanUpDownloadFolder() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("clean_up_download_folder_caller", { gameInstall: game_install, force: true }).then((message) => {
                // Show user notificatio if mod install completed.
                ElNotification({
                    title: `Done`,
                    message: `Done`,
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
        async clearFlightCorePersistentStore() {
            // Clear store...
            await persistentStore.clear();
            // ...and save
            await persistentStore.save();
        }
    }
});
</script>

<style scoped>
.fc_repair__container {
    max-width: 1200px;
    padding: 20px 30px;
    margin: 0 auto;
    color: white;
    position: relative;
}

h3:first-of-type {
    margin-top: 0;
    margin-bottom: 1em;
    text-transform: uppercase;
    font-weight: unset;
}

.el-input {
    width: 50%;
}
</style>
