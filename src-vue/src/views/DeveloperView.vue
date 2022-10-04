<template>
    <div class="fc__developer__container">
        <el-button type="primary" @click="disableDevMode">
            Disable developer mode
        </el-button>

        <el-button type="primary" @click="crashApplication">
            Panic button
        </el-button>

        <el-button type="primary" @click="toggleReleaseCandidate">
            Toggle Release Candidate
        </el-button>
    </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { _get_northstar_version_number } from "../plugins/store"

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
        async toggleReleaseCandidate() {
            // Flip between RELEASE and RELEASE_CANDIDATE
            if (this.$store.state.release_canal === ReleaseCanal.RELEASE) {
                this.$store.state.release_canal = ReleaseCanal.RELEASE_CANDIDATE;
            }
            else {
                this.$store.state.release_canal = ReleaseCanal.RELEASE;
            }

            // Update current state so that update check etc can be performed
            _get_northstar_version_number(this.$store.state);

            // Display notification to highlight change
            ElNotification({
                title: `${this.$store.state.release_canal}`,
                message: `Switched release channel to: "${this.$store.state.release_canal}"`,
                type: 'error',
                position: 'bottom-right'
            });
        }
    }
});
</script>

<style scoped>
.fc__developer__container {
    padding: 20px 30px;
}
</style>
