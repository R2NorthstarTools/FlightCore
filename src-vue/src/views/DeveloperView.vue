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
                    :key="item.value"
                    :label="item.label"
                    :value="item"
                />
            </el-select>
            <el-select v-model="secondTag" class="m-2" placeholder="Second tag">
                <el-option
                    v-for="item in ns_release_tags"
                    :key="item.value"
                    :label="item.label"
                    :value="item"
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

            <el-button type="primary" @click="launchGameViaSteam">
                Launch Northstar via Steam
            </el-button>

            <h3>Mod install:</h3>

            <el-input v-model="mod_to_install_field_string" placeholder="Please input Thunderstore dependency string (example: AuthorName-ModName-1.2.3)" clearable />

            <el-button type="primary" @click="installMod">
                Install mod
            </el-button>

            <h3>Repair:</h3>


            <el-button type="primary" @click="getInstalledMods">
                Get installed mods
            </el-button>

            <h3>Testing</h3>
            <pull-requests-selector />
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { GameInstall } from "../utils/GameInstall";
import { TagWrapper } from "../../../src-tauri/bindings/TagWrapper";
import PullRequestsSelector from "../components/PullRequestsSelector.vue";
import { showNotification } from "../utils/ui";

export default defineComponent({
    name: "DeveloperView",
    components: {
        PullRequestsSelector
    },
    data() {
        return {
            mod_to_install_field_string : "",
            release_notes_text : "",
            first_tag: { label: '', value: {name: ''} },
            second_tag: { label: '', value: {name: ''} },
            ns_release_tags: [] as TagWrapper[],
        }
    },
    computed: {
        firstTag: {
            get(): TagWrapper {
                return this.first_tag;
            },
            set(value: TagWrapper) {
                this.first_tag = value;
            }
        },
        secondTag: {
            get(): TagWrapper {
                return this.second_tag;
            },
            set(value: TagWrapper) {
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
            showNotification('Error', "Never should have been able to get here!", 'error');
        },
        async checkLinuxCompatibility() {
            await invoke("linux_checks")
                .then(() => {
                    showNotification('Linux compatible', 'All checks passed');
                })
                .catch((error) => {
                    showNotification('Not Linux compatible', error, 'error');
                    console.error(error);
                });
        },
        async launchGameWithoutChecks() {
            this.$store.commit('launchGame', true);
        },
        async launchGameViaSteam() {
            this.$store.commit('launchGameSteam', true);
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
                showNotification('Success');
            })
                .catch((error) => {
                    showNotification('Error', error, 'error');
                });
        },
        async installMod() {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            let mod_to_install = this.mod_to_install_field_string;
            await invoke<string>("install_mod_caller", { gameInstall: game_install, thunderstoreModString: mod_to_install }).then((message) => {
                // Show user notification if mod install completed.
                showNotification(`Installed ${mod_to_install}`, message);
            })
                .catch((error) => {
                    showNotification('Error', error, 'error');
                });
        },
        async getTags() {
            await invoke<TagWrapper[]>("get_list_of_tags")
                .then((message) => {
                    this.ns_release_tags = message;
                    showNotification("Done", "Fetched tags");
                })
                .catch((error) => {
                    showNotification('Error', error, 'error');
                });
        },
        async compareTags() {
            await invoke<string>("compare_tags", {firstTag: this.firstTag.value, secondTag: this.secondTag.value})
                .then((message) => {
                    this.release_notes_text = message;
                    showNotification("Done", "Generated release notes");
                })
                .catch((error) => {
                    showNotification('Error', error, 'error');
                });
        },
    }
});
</script>

<style scoped>
</style>
