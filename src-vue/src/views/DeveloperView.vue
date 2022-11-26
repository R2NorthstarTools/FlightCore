<template>
    <div class="fc__developer__container">
        <h3>Basic:</h3>

        <el-button type="primary" @click="disableDevMode">
            Disable developer mode
        </el-button>

        <el-button type="primary" @click="crashApplication">
            Panic button
        </el-button>

        <h3>Linux:</h3>

        <el-button type="primary" @click="checkLinuxCompatibility">
            Check NSProton Compatibility
        </el-button>

        <h3>Testing:</h3>

        <el-button type="primary" @click="toggleReleaseCandidate">
            Toggle Release Candidate
        </el-button>


        <h3>Mod install:</h3>

        <el-input v-model="mod_to_install_field_string" placeholder="Please input Thunderstore dependency string" clearable />

        <el-button type="primary" @click="installMod">
            Install mod
        </el-button>

        <h3>Repair:</h3>

        <el-button type="primary" @click="getInstalledMods">
            Get installed mods
        </el-button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { GameInstall } from "../utils/GameInstall";
import { Store } from 'tauri-plugin-store-api';
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: "DeveloperView",
    data() {
        return {
            mod_to_install_field_string : "",
        }
    },
    methods: {
        disableDevMode() {
            this.$store.commit('toggleDeveloperMode');
        },
        async crashApplication() {
            await invoke("force_panic");
            ElNotification({
                title: 'Error',
                message: "Never should have been able to get here!",
                type: 'error',
                position: 'bottom-right'
            });
        },
        async checkLinuxCompatibility() {
            await invoke("linux_checks")
                .then(() => {
                    ElNotification({
                        title: 'Linux compatible',
                        message: 'All checks passed',
                        type: 'success',
                        position: 'bottom-right'
                    });
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Not linux compatible',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                    console.error(error);
                });
        },
        async toggleReleaseCandidate() {
            // Flip between RELEASE and RELEASE_CANDIDATE
            this.$store.state.northstar_release_canal = this.$store.state.northstar_release_canal === ReleaseCanal.RELEASE
                ? ReleaseCanal.RELEASE_CANDIDATE
                : ReleaseCanal.RELEASE;

            // Save change in persistent store
            await persistentStore.set('northstar-release-canal', { value: this.$store.state.northstar_release_canal });

            // Update current state so that update check etc can be performed
            this.$store.commit("checkNorthstarUpdates");

            console.log(this.$store.state)

            // Display notification to highlight change
            ElNotification({
                title: `${this.$store.state.northstar_release_canal}`,
                message: `Switched release channel to: "${this.$store.state.northstar_release_canal}"`,
                type: 'success',
                position: 'bottom-right'
            });
        },
        async getInstalledMods() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("get_installed_mods_caller", { gameInstall: game_install }).then((message) => {
                // Simply console logging for now
                // In the future we should display the installed mods somewhere
                console.log(message);

                // Just a visual indicator that it worked
                ElNotification({
                    title: 'Success',
                    message: "Success",
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
        async installMod() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            let mod_to_install = this.mod_to_install_field_string;
            await invoke("install_mod_caller", { gameInstall: game_install, thunderstoreModString: mod_to_install }).then((message) => {
                // Show user notificatio if mod install completed.
                ElNotification({
                    title: `Installed ${mod_to_install}`,
                    message: message as string,
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
        }
    }
});
</script>

<style scoped>
.fc__developer__container {
    padding: 20px 30px;
    color: white;
    position: relative;
}
</style>
