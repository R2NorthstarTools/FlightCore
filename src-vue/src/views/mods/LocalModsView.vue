<template>
    <!-- Message displayed if no mod matched searched words -->
    <div v-if="mods.length === 0" class="noModMessage">
        {{ $t('mods.local.no_mods') }}
    </div>

    <el-scrollbar v-else>
        <el-card shadow="hover" v-for="mod in mods" v-bind:key="mod.name">
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
                :title="$t('mods.local.part_of_ts_mod')"
                src="/src/assets/thunderstore-icon.png"
                class="image"
                height="16"
            />
        </el-card>
    </el-scrollbar>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api';
import { defineComponent } from 'vue';
import { GameInstall } from '../../utils/GameInstall';
import { NorthstarMod } from "../../../../src-tauri/bindings/NorthstarMod";
import { showNotification } from '../../utils/ui';

export default defineComponent({
    name: 'LocalModsView',
    computed: {
        installedMods(): NorthstarMod[] {
            return this.$store.state.installed_mods;
        },
        searchValue(): string {
            return this.$store.getters.searchWords;
        },
        mods(): NorthstarMod[] {
            if (this.searchValue.length === 0) {
                return this.installedMods;
            }

            return this.installedMods.filter((mod: NorthstarMod) => {
                return mod.name.toLowerCase().includes(this.searchValue);
            });
        }
    },
    data() {
        return {
            global_load_indicator: false,
        };
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
                showNotification(
                    this.$t('generic.error'),
                    `${error}`,
                    'error'
                );
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
                    showNotification(this.$t('mods.local.success_deleting', {modName: mod.name}));
                })
                .catch((error) => {
                    showNotification(this.$t('generic.error'), error, 'error');
                })
                .finally(() => {
                    this.$store.commit('loadInstalledMods');
                });
        },
    },
    mounted() {
        this.$store.commit('loadInstalledMods');
    }
})
</script>

<style scoped>

</style>
