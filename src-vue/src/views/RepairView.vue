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
import { defineComponent } from "vue";
import { GameInstall } from "../utils/GameInstall";
import { invoke } from "@tauri-apps/api";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { Store } from 'tauri-plugin-store-api';
import { showNotification } from "../utils/ui";
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
                    showNotification('Success', "Disabled all mods but core");
                })
                .catch((error) => {
                    showNotification('Error', error, 'error');
                });
        },
        async forceInstallNorthstar() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;

            // Send notification telling the user to wait for the process to finish
            const notification = showNotification(
                'Force reinstalling Northstar',
                'Please wait',
                'info',
                0
            );

            let install_northstar_result = invoke("install_northstar_caller", { gamePath: game_install.game_path, northstarPackageName: ReleaseCanal.RELEASE });
            await install_northstar_result
                .then((message) => {
                    // Send notification
                    showNotification('Done', `Successfully reinstalled Northstar`);
                    this.$store.commit('checkNorthstarUpdates');
                })
                .catch((error) => {
                    showNotification('Error', error, 'error');
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
                showNotification('Done', 'Done');
            })
                .catch((error) => {
                    showNotification('Error', error, 'error');
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
