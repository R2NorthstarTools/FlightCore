<template>
    <div>
        <el-collapse @change="onChange">
            <el-collapse-item title="Launcher PRs" name="1">
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
                <el-card v-else shadow="hover" v-for="pull_request in pull_requests_launcher"
                    v-bind:key="pull_request.url">
                    <el-button type="primary" @click="installLauncherPR(pull_request)">Install</el-button>
                    <el-button type="primary" @click="downloadLauncherPR(pull_request)">Download</el-button>
                    <a target="_blank" :href="pull_request.html_url">
                        {{ pull_request.number }}: {{ pull_request.title }}
                    </a>
                </el-card>
            </el-collapse-item>

            <el-collapse-item title="Mods PRs" name="2">
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
                <el-card v-else shadow="hover" v-for="pull_request in pull_requests_mods" v-bind:key="pull_request.url">
                    <el-button type="primary" @click="installModsPR(pull_request)">Install</el-button>
                    <el-button type="primary" @click="downloadModsPR(pull_request)">Download</el-button>
                    <a target="_blank" :href="pull_request.html_url">
                        {{ pull_request.number }}: {{ pull_request.title }}
                    </a>
                </el-card>
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
    computed: {
        pull_requests_launcher(): PullsApiResponseElement[] {
            return this.$store.state.pullrequests.pull_requests_launcher;
        },
        pull_requests_mods(): PullsApiResponseElement[] {
            return this.$store.state.pullrequests.pull_requests_mods;
        },
    },
    methods: {
        onChange(e: Object) {
            const openedCollapseNames = Object.values(e);
            if (openedCollapseNames.includes('1') && this.pull_requests_launcher.length === 0) {
                this.getPullRequests('LAUNCHER');
            }
            if (openedCollapseNames.includes('2') && this.pull_requests_mods.length === 0) {
                this.getPullRequests('MODS');
            }
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
</style>
