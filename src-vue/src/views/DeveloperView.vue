<template>
    <div class="fc-container">
        <el-scrollbar>
            <el-alert title="Warning" type="warning" :closable="false" show-icon>
                This page is designed for developers. Some of the buttons here can break your Northstar install if you do not know what you're doing!
            </el-alert>

            <h3>Basic:</h3>

            <el-button type="primary" @click="disableDevMode">
                Disable developer mode
            </el-button>

            <el-button type="primary" @click="crashApplication">
                Panic button
            </el-button>

            <h3>Linux:</h3>

            <el-button type="primary" @click="installNSProton">
                Install NSProton
            </el-button>

            <el-button type="primary" @click="uninstallNSProton">
                Remove NSProton
            </el-button>

            <el-button type="primary" @click="getLocalNSProtonVersion">
                Get local NSProton Version
            </el-button>

            <h3>Testing:</h3>

            <el-button type="primary" @click="launchGameWithoutChecks">
                Launch Northstar (bypass all checks)
            </el-button>

            <el-button type="primary" @click="launchGameViaSteam">
                Launch Northstar via Steam
            </el-button>

            <el-button type="primary" @click="installLauncherGitMain">
                Install launcher from main branch
            </el-button>

            <br />
            <br />

            <el-button type="primary" @click="getAvailableNorthstarVersions">
                Get available versions
            </el-button>

            <el-select v-model="selected_ns_version" class="m-2" placeholder="Versions">
                <el-option
                    v-for="item in ns_versions"
                    :key="item.value"
                    :label="item.label"
                    :value="item"
                />
            </el-select>

            <el-button type="primary" @click="installNorthstarVersion">
                Install
            </el-button>

            <h3>Repair:</h3>

            <el-button type="primary" @click="checkCgnat">
                Run tracert and collect hop count
            </el-button>

            <el-button type="primary" @click="getInstalledMods">
                Get installed mods
            </el-button>

            <h3>Testing</h3>
            <pull-requests-selector />

            <h3>Mod install:</h3>

            <el-input v-model="mod_to_install_field_string" placeholder="Please input Thunderstore dependency string (example: AuthorName-ModName-1.2.3)" clearable />

            <el-button type="primary" @click="installMod">
                Install mod
            </el-button>

            <h3>Release management</h3>
            <el-select v-model="selected_project" placeholder="Select">
            <el-option
                v-for="item in project"
                :key="item.value"
                :label="item.label"
                :value="item.value"
                />
            </el-select>
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

            <el-button type="primary" @click="copyReleaseNotesToClipboard">
                Copy to clipboard
            </el-button>

            <el-input
                v-model="release_notes_text"
                type="textarea"
                :rows="5"
                placeholder="Output"
            />

            <h3>Release announcements</h3>

            <el-button type="primary" @click="generateReleaseAnnouncementMessage">
                Generate release announcement
            </el-button>

            <el-input
                v-model="discord_release_announcement_text"
                type="textarea"
                :rows="5"
                placeholder="Output"
            />
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { NorthstarLaunchOptions } from "../../../src-tauri/bindings/NorthstarLaunchOptions";
import { TagWrapper } from "../../../src-tauri/bindings/TagWrapper";
import { NorthstarThunderstoreReleaseWrapper } from "../../../src-tauri/bindings/NorthstarThunderstoreReleaseWrapper";
import PullRequestsSelector from "../components/PullRequestsSelector.vue";
import { showErrorNotification, showNotification } from "../utils/ui";
import { Project } from "../../../src-tauri/bindings/Project"

export default defineComponent({
    name: "DeveloperView",
    components: {
    },
    data() {
        return {
            mod_to_install_field_string: "",
            release_notes_text: "",
            discord_release_announcement_text: "",
            first_tag: { label: '', value: { name: '' } },
            second_tag: { label: '', value: { name: '' } },
            ns_release_tags: [] as TagWrapper[],
            ns_versions: [] as NorthstarThunderstoreReleaseWrapper[],
            selected_ns_version: { label: '', value: { package: '', version: '' } } as NorthstarThunderstoreReleaseWrapper,
            selected_project: "FlightCore",
            project: [
                {
                    value: 'FlightCore',
                    label: 'FlightCore',
                },
                {
                    value: 'Northstar',
                    label: 'Northstar',
                }
            ],
        }
    },
});
</script>

<style scoped>
</style>
