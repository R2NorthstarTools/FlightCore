<template>
    <div class="fc-container">
        <el-scrollbar>
            <el-alert title="Warning" type="warning" :closable="false" show-icon>
                This page is designed for developers. Some of the buttons here can break your Northstar install if you do not know what you're doing!
            </el-alert>

            <el-button type="primary" @click="getTags">
                Get tags
            </el-button>

            <el-select v-model="firstTag" class="m-2" placeholder="First tag">
                <el-option
                    v-for="item in ns_release_tags"
                    :key="item.name"
                    :label="item.name"
                    :value="item.name"
                />
            </el-select>
            <el-select v-model="secondTag" class="m-2" placeholder="Second tag">
                <el-option
                    v-for="item in ns_release_tags"
                    :key="item.name"
                    :label="item.name"
                    :value="item.name"
                />
            </el-select>

            <el-button type="primary" @click="compareTags">
                Compare Tags
            </el-button>

            <el-input
                v-model="release_notes_text"
                type="textarea"
                :rows="5"
                placeholder="Output"
            />

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

            <el-button type="primary" @click="launchGameWithoutChecks">
                Launch Northstar (bypass all checks)
            </el-button>

            <h3>Mod install:</h3>

            <el-input v-model="mod_to_install_field_string" placeholder="Please input Thunderstore dependency string (example: AuthorName-ModName-1.2.3)" clearable />

            <el-button type="primary" @click="installMod">
                Install mod
            </el-button>

            <h3>Repair:</h3>

            <el-button type="primary" @click="disableAllModsButCore">
                Disable all but core mods
            </el-button>

            <el-button type="primary" @click="getInstalledMods">
                Get installed mods
            </el-button>

            <el-button type="primary" @click="cleanUpDownloadFolder">
                Force delete temp download folder
            </el-button>

            <el-button type="primary" @click="clearFlightCorePersistentStore">
                Delete FlightCore persistent store
            </el-button>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";
import { GameInstall } from "../utils/GameInstall";
import { Store } from 'tauri-plugin-store-api';
import { Tag } from "../../../src-tauri/bindings/Tag";
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: "DeveloperView",
    data() {
        return {
            mod_to_install_field_string: "",
            release_notes_text : "",
            first_tag:  { name: '' },
            second_tag:  { name: '' },
            ns_release_tags: [] as Tag[],
        }
    },
    computed: {
        ns_release_tags(): Tag[] {
            return this.ns_release_tags;
        },
        firstTag: {
            get(): Tag {
                return this.first_tag;
            },
            set(value: Tag) {
                this.first_tag = value;
            }
        },
        secondTag: {
            get(): Tag {
                return this.second_tag;
            },
            set(value: Tag) {
                this.second_tag = value;
            }
        },
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
        async launchGameWithoutChecks() {
            this.$store.commit('launchGame', true);
        },
        async disableAllModsButCore() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("disable_all_but_core", { gameInstall: game_install }).then((message) => {
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
        async getInstalledMods() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            await invoke("get_installed_mods_and_properties", { gameInstall: game_install }).then((message) => {
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
                // Show user notification if mod install completed.
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
        async getTags() {
            await invoke<Tag[]>("get_list_of_tags")
                .then((message) => {
                    this.ns_release_tags = message;
                    ElNotification({
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
        async compareTags() {
            await invoke<string>("compare_tags", {firstTag: this.firstTag, secondTag: this.secondTag})
                .then((message) => {
                    this.release_notes_text = message;
                    ElNotification({
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
    }
});
</script>

<style scoped>
</style>
