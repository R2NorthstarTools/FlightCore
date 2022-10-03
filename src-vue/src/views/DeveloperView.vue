<template>
    <div class="fc__developer__container">
        <el-button type="primary" @click="disableDevMode">
            Disable developer mode
        </el-button>

        <el-button type="primary" @click="crashApplication">
            Panic button
        </el-button>
    </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";

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
        }
    }
});
</script>

<style scoped>
.fc__developer__container {
    padding: 20px 30px;
}
</style>
