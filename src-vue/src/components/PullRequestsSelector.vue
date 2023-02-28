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
import { invoke } from "@tauri-apps/api";
import { ElNotification } from "element-plus";

export default defineComponent({
    name: 'PullRequestsSelector',
    computed: {
        pull_requests_launcher(): PullsApiResponseElement[] {
            return this.$store.state.pull_requests_launcher;
        },
        pull_requests_mods(): PullsApiResponseElement[] {
            return this.$store.state.pull_requests_mods;
        },
    },
    methods: {
        onChange(e: string) {
            if (e === '1' && this.pull_requests_launcher.length === 0) {
                this.getPullRequests('LAUNCHER');
            } else if (e === '2' && this.pull_requests_mods.length === 0) {
                this.getPullRequests('MODS');
            }
        },
        async getPullRequests(pull_request_type: PullRequestType) {
            await invoke<PullsApiResponseElement[]>("get_pull_requests_wrapper", { installType: pull_request_type })
                .then((message) => {
                    switch (pull_request_type) {
                        case "MODS":
                            this.$store.state.pull_requests_mods = message;
                            break;

                        case "LAUNCHER":
                            this.$store.state.pull_requests_launcher = message;
                            break;

                        default:
                            console.error("We should never end up here");
                    }
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
        async installLauncherPR(pull_request: PullsApiResponseElement) {
            // Send notification telling the user to wait for the process to finish
            const notification = ElNotification({
                title: `Installing launcher PR ${pull_request.number}`,
                message: 'Please wait',
                duration: 0,
                type: 'info',
                position: 'bottom-right'
            });

            await invoke("apply_launcher_pr", { pullRequest: pull_request, gameInstallPath: this.$store.state.game_path })
                .then((message) => {
                    console.log(message);
                    // Show user notification if mod install completed.
                    ElNotification({
                        title: `Done`,
                        message: `Installed ${pull_request.number}: "${pull_request.title}"`,
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
                    // Clear old notification
                    notification.close();
                });
        },
        async installModsPR(pull_request: PullsApiResponseElement) {
            // Send notification telling the user to wait for the process to finish
            const notification = ElNotification({
                title: `Installing mods PR ${pull_request.number}`,
                message: 'Please wait',
                duration: 0,
                type: 'info',
                position: 'bottom-right'
            });

            await invoke("apply_mods_pr", { pullRequest: pull_request, gameInstallPath: this.$store.state.game_path })
                .then((message) => {
                    // Show user notification if mod install completed.
                    ElNotification({
                        title: `Done`,
                        message: `Installed ${pull_request.number}: "${pull_request.title}"\nMake sure to launch via batch file or by specifying correct profile!`,
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
                    // Clear old notification
                    notification.close();
                });
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
