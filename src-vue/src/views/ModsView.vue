<template>
    <div class="fc__mods__container">
        <h3>Installed Mods:</h3>
        <div>
            <el-card shadow="hover" v-for="mod in releases">
                <el-switch style="--el-switch-on-color: #13ce66; --el-switch-off-color: #8957e5" v-model="mod.enabled" disabled />
                {{mod.name}}
            </el-card>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { invoke } from '@tauri-apps/api/tauri';
import { GameInstall } from "../utils/GameInstall";

export default defineComponent({
    name: "ModsView",
    data() {
        return {
            releases: [],
        }
    },
    async mounted() {
        let game_install = {
            game_path: this.$store.state.game_path,
            install_type: this.$store.state.install_type
        } as GameInstall;
        // Call back-end for installed mods
        this.releases = await invoke("get_installed_mods_caller", { gameInstall: game_install });
        console.log(this.releases);
    }
});
</script>

<style scoped>
.fc__mods__container {
    padding: 20px 30px;
    color: white;
}
</style>
