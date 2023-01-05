<template>
    <div class="fc-container" style="display: flex">
        <!-- Local mods/Thunderstore mods menu -->
        <nav class="fc_mods__menu">
            <el-menu
                default-active="1"
                text-color="#fff"
            >
                <h5>Mods</h5>
                <el-menu-item index="1" @click="show_local_mods = true">
                    <el-icon><Folder /></el-icon>
                    <span>Local</span>
                </el-menu-item>
                <el-menu-item index="2" @click="show_local_mods = false">
                    <el-icon><Connection /></el-icon>
                    <span>Online</span>
                </el-menu-item>

                <!-- Search inputs -->
                <h5>Filter</h5>
                <el-input v-model="input" placeholder="Search" clearable @input="onFilterTextChange" />
                <el-select
                    v-if="!show_local_mods"
                    v-model="modCategories"
                    multiple
                    placeholder="Select categories"
                >
                    <el-option
                        v-for="item in $store.state.thunderstoreModsCategories"
                        :key="item"
                        :label="item"
                        :value="item"
                    />
                </el-select>

            </el-menu>
        </nav>

        <!-- Mods content -->
        <div class="fc_mods__container">
            <el-scrollbar v-if="show_local_mods">
                <h3>Installed Mods:</h3>
                <div>
                    <p v-if="installedMods.length === 0">No mods were found.</p>
                    <el-card v-else shadow="hover" v-for="mod in installedMods" v-bind:key="mod.name">
                        <el-switch style="--el-switch-on-color: #13ce66; --el-switch-off-color: #8957e5" v-model="mod.enabled"
                                   :before-change="() => updateWhichModsEnabled(mod)" :loading="global_load_indicator" />
                        <el-popconfirm
                            title="Are you sure to delete this mod?"
                            @confirm="deleteMod(mod)"
                        >
                            <template #reference>
                                <el-button type="danger">Delete</el-button>
                            </template>
                        </el-popconfirm>
                        {{ mod.name }}
                    </el-card>
                </div>
            </el-scrollbar>

            <thunderstore-mods-view 
                v-else 
                :input="input"
                :searchValue="searchValue"
                :selectedCategories="modCategories"
            />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from "element-plus";
import { invoke } from '@tauri-apps/api/tauri';
import { GameInstall } from "../utils/GameInstall";
import { NorthstarMod } from "../utils/NorthstarMod"
import ThunderstoreModsView from "./ThunderstoreModsView.vue";

export default defineComponent({
    name: "ModsView",
    components: { ThunderstoreModsView },
    data() {
        return {
            global_load_indicator: false,
            show_local_mods: true,

            // This is the model for the search input.
            input: '',
            // This is the treated value of search input
            searchValue: '',
            // Selected mod categories
            modCategories: []
        }
    },
    async mounted() {
        this.$store.commit('loadInstalledMods');
    },
    computed: {
        installedMods(): NorthstarMod[] {
            return this.$store.state.installed_mods;
        }
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
        },
        async deleteMod(mod: NorthstarMod) {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("delete_northstar_mod", { gameInstall: game_install, nsmodName: mod.name })
                .then((message) => {
                    // Just a visual indicator that it worked
                    ElNotification({
                        title: `Success deleting ${mod.name}`,
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
                })
                .finally(() => {
                    this.$store.commit('loadInstalledMods');
                });
        },

        /**
         * This method is called each time search input is modified, and
         * triggered filtered mods recomputing by updating the `searchValue`
         * variable.
         *
         * This converts research string and all researched fields to
         * lower case, to match mods regardless of font case.
         */
         onFilterTextChange(value: string) {
            this.searchValue = value.toLowerCase();
        },
    }
});
</script>

<style>
.fc_mods__menu {
    display: flex;
    max-width: 222px;
    min-width: 222px;
    padding: 10px;
}

.fc_mods__container {
    display: flex;
    width: 100%;
    flex-direction: column;
}

.fc_mods__menu h5 {
    margin: 8px 0 16px 5px;
}

.fc_mods__menu h5:not(:first-child){
    margin-top: 32px;
}

.fc_mods__menu > .el-menu {
    background-color: transparent;
    border: none;
    width: 100%;
}

.fc_mods__menu > .el-menu > .el-menu-item {
    height: 32px;
    margin-bottom: 5px;
    border-radius: 5px;
    color: #e2e6e7;
}

.fc_mods__menu > .el-menu > .el-menu-item:hover {
    background-color: #4e4e4e3b;
}

.fc_mods__menu > .el-menu > .el-menu-item.is-active {
    color: white;
    background-color: #4e4e4e7a;
}

.el-select {
    width: 100%;
    margin-top: 5px;
}
</style>
