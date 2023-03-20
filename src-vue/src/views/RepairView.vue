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

            <el-button type="primary" @click="forceInstallNorthstar">
                Force reinstall Northstar
            </el-button>

            <h2>FlightCore</h2>

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
import { appWindow } from '@tauri-apps/api/window';
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { GameInstall } from "../utils/GameInstall";
import { invoke } from "@tauri-apps/api";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { Store } from 'tauri-plugin-store-api';
const persistentStore = new Store('flight-core-settings.json');

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
        async forceInstallNorthstar() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;

            // Send notification telling the user to wait for the process to finish
            const notification = ElNotification({
                title: 'Force reinstalling Northstar',
                message: 'Please wait',
                duration: 0,
                type: 'info',
                position: 'bottom-right'
            });

            let install_northstar_result = invoke("install_northstar_caller", { gamePath: game_install.game_path, northstarPackageName: ReleaseCanal.RELEASE });

            const unlistenProgress = await appWindow.listen(
                'northstar-install-download-progress',
                ({ event, payload }) => console.log(payload)
            );
            await install_northstar_result
                .then((message) => {
                    // Send notification
                    ElNotification({
                        title: `Done`,
                        message: `Successfully reinstalled Northstar`,
                        type: 'success',
                        position: 'bottom-right'
                    });
                    this.$store.commit('checkNorthstarUpdates');
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                    console.error(error);
                })
                .finally(() => {
                    // Clear old notification
                    notification.close();
                });
        },
        async cleanUpDownloadFolder() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("clean_up_download_folder_caller", { gameInstall: game_install, force: true }).then((message) => {
                // Show user notification if task completed.
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
        },
    }
});
</script>

<style scoped>
.fc-container {
    padding-top: 0px;
}
</style>
