<template>
    <div class="fc__developer__container">
        <el-button type="primary" @click="disableDevMode">
            Disable developer mode
        </el-button>

        <el-button type="primary" @click="crashApplication">
            Panic button
        </el-button>

        <el-button type="primary" @click="checkLinuxCompatability">
            Check NSProton Compatability
        </el-button>

        <el-button type="primary" @click="toggleReleaseCandidate">
            Toggle Release Candidate
        </el-button>

        <h3>Repair:</h3>

        <el-button type="primary" @click="disableAllModsButCore">
            Disable all but core mods
        </el-button>

    </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { GameInstall } from "../utils/GameInstall";

export default defineComponent({
    name: "DeveloperView",
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
        async checkLinuxCompatability() {
            let LinuxCompatabile = await invoke("linux_checks");
            if (!LinuxCompatabile) {
                ElNotification({
                title: 'Not linux compatabile',
                message: 'GLIBC is not version 2.33 or greater',
                type: 'error',
                position: 'bottom-right'
                });
            } else {
                ElNotification({
                title: 'Linux compatabile',
                message: 'No error reported',
                type: 'success',
                position: 'bottom-right'
                });
            }
        },
        async toggleReleaseCandidate() {
            // Flip between RELEASE and RELEASE_CANDIDATE
            this.$store.state.release_canal = this.$store.state.release_canal === ReleaseCanal.RELEASE
                ? ReleaseCanal.RELEASE_CANDIDATE
                : ReleaseCanal.RELEASE;

            // Update current state so that update check etc can be performed
            this.$store.commit("checkNorthstarUpdates");

            console.log(this.$store.state)

            // Display notification to highlight change
            ElNotification({
                title: `${this.$store.state.release_canal}`,
                message: `Switched release channel to: "${this.$store.state.release_canal}"`,
                type: 'success',
                position: 'bottom-right'
            });
        },
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
        }
    }
});
</script>

<style scoped>
.fc__developer__container {
    padding: 20px 30px;
    color: white;
}
</style>
