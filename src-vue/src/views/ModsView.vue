<template>
    <div class="fc-container">
        <el-scrollbar>
            <h3>Installed Mods:</h3>
            <div>
                <el-card shadow="hover" v-for="mod in $store.state.installed_mods">
                    <el-switch style="--el-switch-on-color: #13ce66; --el-switch-off-color: #8957e5" v-model="mod.enabled"
                        :before-change="() => updateWhichModsEnabled(mod)" :loading="global_load_indicator" />
                    {{mod.name}}
                </el-card>
            </div>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { invoke } from '@tauri-apps/api/tauri';
import { GameInstall } from "../utils/GameInstall";
import { NorthstarMod } from "../utils/NorthstarMod"

export default defineComponent({
    name: "ModsView",
    data() {
        return {
            global_load_indicator: false
        }
    },
    async mounted() {
        this.$store.commit('loadInstalledMods');
    },
    methods: {
        async updateWhichModsEnabled(mod: NorthstarMod) {
            this.global_load_indicator = true;

            // Setup up struct
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;

            // enable/disable specific mod
            try {
                await invoke("set_mod_enabled_status_caller", {
                    gameInstall: game_install,
                    modName: mod.name,
                    // Need to set it to the opposite of current state,
                    // as current state is only updated after command is run
                    isEnabled: !mod.enabled,
                })
            }
            catch (error) {
                ElNotification({
                    title: 'Error',
                    message: `${error}`,
                    type: 'error',
                    position: 'bottom-right'
                });
                this.global_load_indicator = false;
                return false;
            }

            this.global_load_indicator = false;
            return true;
        }
    }
});
</script>

<style>
</style>
