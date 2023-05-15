<template>
    <el-card shadow="hover">
        <el-switch style="--el-switch-on-color: #13ce66; --el-switch-off-color: #8957e5" v-model="mod.enabled"
                    :before-change="() => updateWhichModsEnabled(mod)" :loading="global_load_indicator" />
        <el-popconfirm
            :title="$t('mods.local.delete_confirm')"
            :confirm-button-text="$t('generic.yes')"
            :cancel-button-text="$t('generic.no')"
            @confirm="deleteMod(mod)"
        >
            <template #reference>
                <el-button type="danger">
                    {{ $t('mods.local.delete') }}
                </el-button>
            </template>
        </el-popconfirm>
        {{ mod.name }}
        <span v-if="mod.version != null">(v{{ mod.version }})</span>
        <img
            v-if="mod.thunderstore_mod_string != null"
            :title="$t('mods.local.part_of_ts_mod') + '\n' + mod.thunderstore_mod_string"
            src="/src/assets/thunderstore-icon.png"
            class="image"
            height="16"
        />
    </el-card>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { NorthstarMod } from "../../../src-tauri/bindings/NorthstarMod";
import { GameInstall } from "../utils/GameInstall";
import { showErrorNotification, showNotification } from "../utils/ui";

export default defineComponent({
    name: "LocalModCard",
    props: {
        mod: {
            required: true,
            type: Object as () => NorthstarMod
        }
    },
    data() {
        return {
            global_load_indicator: false,
        };
    },
    computed: {
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
                await invoke("set_mod_enabled_status", {
                    gameInstall: game_install,
                    modName: mod.name,
                    // Need to set it to the opposite of current state,
                    // as current state is only updated after command is run
                    isEnabled: !mod.enabled,
                })
            }
            catch (error) {
                showErrorNotification(`${error}`);
                this.global_load_indicator = false;
                return false;
            }

            this.global_load_indicator = false;
            return true;
        },
        async deleteMod(mod: NorthstarMod) {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("delete_northstar_mod", { gameInstall: game_install, nsmodName: mod.name })
                .then((message) => {
                    // Just a visual indicator that it worked
                    showNotification(this.$t('mods.local.success_deleting', { modName: mod.name }));
                })
                .catch((error) => {
                    showErrorNotification(error);
                })
                .finally(() => {
                    this.$store.commit('loadInstalledMods');
                });
        },
    }
});
</script>

<style scoped>

</style>
