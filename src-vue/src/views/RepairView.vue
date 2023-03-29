<template>
    <div class="fc-container">
        <el-scrollbar>
            <el-alert :title="$t('generic.informationShort')" type="info" :closable="false" show-icon>
                {{ $t('settings.repair.window.warning') }}
            </el-alert>

            <h1>{{ $t('settings.repair.title') }}</h1>

            <h2>Northstar</h2>

            <el-button type="primary" @click="disableAllModsButCore">
                {{ $t('settings.repair.window.disable_all_but_core') }}
            </el-button>

            <el-button type="primary" @click="forceInstallNorthstar">
                {{ $t('settings.repair.window.force_reinstall_ns') }}
            </el-button>

            <h2>FlightCore</h2>

            <el-button type="primary" @click="cleanUpDownloadFolder">
                {{ $t('settings.repair.window.force_delete_temp_dl') }}
            </el-button>

            <el-button type="primary" @click="clearFlightCorePersistentStore">
                {{ $t('settings.repair.window.delete_persistent_store') }}
            </el-button>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { GameInstall } from "../utils/GameInstall";
import { InstallProgress } from "../../../src-tauri/bindings/InstallProgress";
import { invoke } from "@tauri-apps/api";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { Store } from 'tauri-plugin-store-api';
import { appWindow } from "@tauri-apps/api/window";
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: "RepairView",
    computed: {
        lang(): string {
            return this.$root!.$i18n.locale;
        }
    },
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
                ({ event, payload }) => {
                    let typed_payload = payload as InstallProgress; // This is bad but don't know how to do it
                    console.log("current_downloaded:", typed_payload.current_downloaded);
                    console.log("total_size:        ", typed_payload.total_size);
                    console.log("state:             ", typed_payload.state);
                }
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
    },
    watch: {
        // Lang value is propagated to repair view after it's mounted, so we need to watch
        // its value, and update window title accordingly.
        lang(newv: string) {
            appWindow.setTitle( this.$t('settings.repair.window.title') );
        }
    }
});
</script>

<style scoped>
.fc-container {
    padding-top: 0px;
}
</style>
