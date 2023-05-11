<template>
    <div>
        <el-collapse @change="onChange">
            <el-collapse-item name="1" @keydown.space="launcherSearchSpace">
                <template #title>
                    Launcher PRs
                    <el-input class="pr_search_input" v-model="launcherSearch" placeholder="Filter pull requests" @click.stop="() =>  false"></el-input>
                </template>

                <p v-if="pull_requests_launcher.length === 0">
                    <el-progress
                        :show-text="false"
                        :percentage="100"
                        status="warning"
                        :indeterminate="true"
                        :duration="1"
                        style="margin: 15px"
                    />
                </p>
                <el-card
                    v-else-if="filtered_launcher_pull_requests.length !== 0"
                    shadow="hover"
                    v-for="pull_request in filtered_launcher_pull_requests"
                    v-bind:key="pull_request.url"
                >
                    <el-button type="primary" @click="installLauncherPR(pull_request)">Install</el-button>
                    <el-button type="primary" @click="downloadLauncherPR(pull_request)">Download</el-button>
                    <a target="_blank" :href="pull_request.html_url">
                        {{ pull_request.number }}: {{ pull_request.title }}
                    </a>
                </el-card>
                <div v-else class="no_matching_pr">
                    No matching PR found.
                </div>
            </el-collapse-item>

            <el-collapse-item name="2" @keydown.space="modsSearchSpace">
                <template #title>
                    Mods PRs
                    <el-input class="pr_search_input" v-model="modsSearch" placeholder="Filter pull requests" @click.stop="() => false"></el-input>
                </template>
                <div style="margin: 15px">
                    <el-alert title="Warning" type="warning" :closable="false" show-icon>
                        Mod PRs are installed into a separate profile. Make sure to launch via
                        'r2ns-launch-mod-pr-version.bat' or via '-profile=R2Northstar-PR-test-managed-folder' to actually
                        run the PR version!
                    </el-alert>
                </div>
                <p v-if="pull_requests_mods.length === 0">
                    <el-progress
                        :show-text="false"
                        :percentage="100"
                        status="warning"
                        :indeterminate="true"
                        :duration="1"
                        style="margin: 15px"
                    />
                </p>
                <el-card
                    v-else-if="filtered_mods_pull_requests.length !== 0"
                    shadow="hover"
                    v-for="pull_request in filtered_mods_pull_requests"
                    v-bind:key="pull_request.url"
                >
                    <el-button type="primary" @click="installModsPR(pull_request)">Install</el-button>
                    <el-button type="primary" @click="downloadModsPR(pull_request)">Download</el-button>
                    <a target="_blank" :href="pull_request.html_url">
                        {{ pull_request.number }}: {{ pull_request.title }}
                    </a>
                </el-card>
                <div v-else class="no_matching_pr">
                    No matching PR found.
                </div>
            </el-collapse-item>
        </el-collapse>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { PullRequestType } from '../../../src-tauri/bindings/PullRequestType';
import { PullsApiResponseElement } from '../../../src-tauri/bindings/PullsApiResponseElement';

export default defineComponent({
    name: 'PullRequestsSelector',
    data: () => ({
        launcherSearch: '',
        modsSearch: ''
    }),
    computed: {
        pull_requests_launcher(): PullsApiResponseElement[] {
            return this.$store.state.pullrequests.pull_requests_launcher;
        },
        pull_requests_mods(): PullsApiResponseElement[] {
            return this.$store.state.pullrequests.pull_requests_mods;
        },

        filtered_launcher_pull_requests(): PullsApiResponseElement[] {
            if (this.launcherSearch.length === 0) {
                return this.pull_requests_launcher;
            }

            return this.pull_requests_launcher.filter(pr =>
                // Check PR id
                pr.number.toString().indexOf(this.launcherSearch) !== -1
                // Check PR title
                || pr.title.toLowerCase().indexOf(this.launcherSearch.toLowerCase()) !== -1);
        },
        filtered_mods_pull_requests(): PullsApiResponseElement[] {
            if (this.modsSearch.length === 0) {
                return this.pull_requests_mods;
            }

            return this.pull_requests_mods.filter(pr =>
                // Check PR id
                pr.number.toString().indexOf(this.modsSearch) !== -1
                // Check PR title
                || pr.title.toLowerCase().indexOf(this.modsSearch.toLowerCase()) !== -1);
        },
    },
    methods: {
        onChange(e: Object) {
            const openedCollapseNames = Object.values(e);
            if (openedCollapseNames.includes('1') && this.pull_requests_launcher.length === 0) {
                this.getPullRequests('Launcher');
            }
            if (openedCollapseNames.includes('2') && this.pull_requests_mods.length === 0) {
                this.getPullRequests('Mods');
            }
        },
        launcherSearchSpace(e: KeyboardEvent) {
            e.preventDefault();
            this.launcherSearch += ' ';
        },
        modsSearchSpace(e: KeyboardEvent) {
            e.preventDefault();
            this.modsSearch += ' ';
        },
        async getPullRequests(pull_request_type: PullRequestType) {
            this.$store.commit('getPullRequests', pull_request_type);
        },
        async downloadLauncherPR(pull_request: PullsApiResponseElement) {
            this.$store.commit('downloadLauncherPR', pull_request);
        },
        async downloadModsPR(pull_request: PullsApiResponseElement) {
            this.$store.commit('downloadModsPR', pull_request);
        },
        async installLauncherPR(pull_request: PullsApiResponseElement) {
            this.$store.commit('installLauncherPR', pull_request);
        },
        async installModsPR(pull_request: PullsApiResponseElement) {
            this.$store.commit('installModsPR', pull_request);
        },
    }
})
</script>

<style scoped>
.el-collapse {
    border-radius: var(--el-border-radius-base);
    overflow: hidden;
}

:deep(.el-collapse-item__header) {
    padding-left: 10px;
    font-size: 14px;
}

.el-collapse:deep(.el-collapse-item__arrow) {
    margin: 0 8px;
}

.pr_search_input {
    width: 200px;
    margin: 0 0 0 auto;
}

.no_matching_pr {
    margin: 0 auto;
    width: max-content;
}
</style>
