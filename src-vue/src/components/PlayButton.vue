<script lang="ts">
import { defineComponent } from 'vue';
import { NorthstarState } from '../utils/NorthstarState';
import { ReleaseCanal } from '../utils/ReleaseCanal';
import { appWindow } from '@tauri-apps/api/window';
import { InstallProgress } from '../../../src-tauri/bindings/InstallProgress';

export default defineComponent({
    name: 'PlayButton',
    computed: {
        currentCanal: {
            get(): ReleaseCanal {
                return this.$store.state.northstar_release_canal;
            },
            set(value: ReleaseCanal) {
                if (value !== this.currentCanal) {
                    this.$store.commit('toggleReleaseCandidate');
                }
            }
        },
        playButtonLabel(): string {
            if (this.$store.state.northstar_is_running) {
                return this.$t("play.button.northstar_is_running");
            }

            switch(this.$store.state.northstar_state) {
                case NorthstarState.GAME_NOT_FOUND:
                    return this.$t("play.button.select_game_dir");
                case NorthstarState.INSTALL:
                    return this.$t("play.button.install");
                case NorthstarState.INSTALLING:
                    return this.$t("play.button.installing");
                case NorthstarState.MUST_UPDATE:
                    return this.$t("play.button.update");
                case NorthstarState.UPDATING:
                    return this.$t("play.button.updating");
                case NorthstarState.READY_TO_PLAY:
                    return this.$t("play.button.ready_to_play");

                default:
                    return "";
            }
        },
        northstarIsRunning(): boolean {
            return this.$store.state.northstar_is_running;
        },
        options(): {key: string, value: string}[] {
            return Object.keys(ReleaseCanal).map(function (v) {
                return {
                    key: v,
                    value: Object.keys(ReleaseCanal)[Object.values(ReleaseCanal).indexOf(v)]
                }
            });
        },
        selectOptions(): {label: string, options: {value: ReleaseCanal, label: string}[]}[] {
            return [
                {
                    label: 'Beta',
                    options: [
                        {
                            value: ReleaseCanal.RELEASE_CANDIDATE,
                            label: this.$t('channels.names.NorthstarReleaseCandidate'),
                        },
                    ]
                },
                {
                    label: 'Stable',
                    options: [
                        {
                            value: ReleaseCanal.RELEASE,
                            label: 'Northstar',
                        },
                    ]
                }
            ];
        },
        showReleaseSwitch(): boolean {
            return this.$store.state.enableReleasesSwitch;
        },

        /**
         * Button has rounded edges on its right only if releases switching is enabled.
         */
        buttonRadiusStyle(): string {
            return this.showReleaseSwitch
                ? 'border-radius: 2px 0 0 2px;'
                : 'border-radius: 2px';
        },
        progressBarStyle(): string {
            return !this.install_or_update ? 'hide-progress' : '';
        }
    },
    data() {
        return {
        percentage: 0,
        color: '#409EFF',
        install_or_update: false,
        status: "unknown",
        current_downloaded: -1,
        total_size: -1,
        };
    },
    methods: {
        formatBytes(bytes: number, decimals = 2) {
            if (bytes === 0) return '0 Bytes';
            const k = 1000;
            const dm = decimals < 0 ? 0 : decimals;
            const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
        },
        formatText() {
            if (this.status == "DOWNLOADING") {
                const current_downloaded_string = this.formatBytes(this.current_downloaded);
                const total_size_string = this.formatBytes(this.total_size);
                const status = this.$t("generic.downloading");
                return `${status}: ${current_downloaded_string}/${total_size_string}`;
            }
            if (this.status == "EXTRACTING") {
                return this.$t("generic.extracting");
            }
            return "Inactive";  // Needed to keep same size format when progress bar is hidden
        },
        async launchGame() {
            let unlistenProgress = await appWindow.listen(
                'northstar-install-download-progress',
                ({ event, payload }) => {
                    this.install_or_update = true;
                    let progress = payload as InstallProgress; // This is bad but don't know how to do it properly
                    if (progress.state == "DOWNLOADING") {
                        this.percentage = ((Number(progress.current_downloaded) / Number(progress.total_size)) * 100);
                        this.color = '#409EFF';
                        this.status = progress.state;
                        this.current_downloaded = Number(progress.current_downloaded);
                        this.total_size = Number(progress.total_size);
                    }
                    if (progress.state == "EXTRACTING") {
                        this.percentage = 100;
                        this.color = '#67C23A';
                        this.status = progress.state;
                    }
                    if (progress.state == "DONE") {
                        // Clear state again
                        this.install_or_update = false
                        this.status = progress.state;
                    }
                }
            );
            this.$store.commit('launchGame');
            this.install_or_update = false;
        }
    }
});
</script>

<template>
    <el-button :disabled="northstarIsRunning"
               type="primary" size="large" @click="launchGame"
               class="fc_launch__button" :style="buttonRadiusStyle">
        {{ playButtonLabel }}
    </el-button>
    <el-select v-if="showReleaseSwitch" :disabled="northstarIsRunning"
               v-model="currentCanal" placeholder="Select">
        <el-option-group
            v-for="group in selectOptions"
            :key="group.label"
            :label="group.label"
        >
            <el-option
                v-for="item in group.options"
                :key="item.value"
                :label="item.label"
                :value="item.value"
            />
        </el-option-group>
    </el-select>
    <el-progress
        :class="progressBarStyle"
        :format="formatText"
        :percentage="percentage"
        :color="color"
        :indeterminate="status === 'EXTRACTING'"
        :duration="1"
    >
    </el-progress>
</template>

<style scoped>
.el-progress {
    margin-top: 10px;
}

/* Set progress bar width */
.el-progress:deep(.el-progress-bar) {
    width: 200px;
}

.hide-progress {
    opacity: 0;
}

button {
    text-transform: uppercase;
    padding: 30px;
    font-size: 15px;
    margin-right: 0;
}
.fc_launch__button:focus {
    background-color: var(--el-color-primary);
    border-color: var(--el-color-primary);
}

/* Release canal selector */

.el-select {
    width: 0;
    margin-right: 50px;
    border-left: 1px solid rgb(176, 205, 255);
}

.el-select:deep(.el-input__wrapper) {
    padding: 0 9px 0 0;
    background-color: var(--el-color-primary);
    border: none;
    border-radius: 0 2px 2px 0;
    height: 62px;
    box-shadow: none !important;
    --el-disabled-bg-color: #a0cfff;
}

.el-select:deep(.el-icon) {
    color: white !important;
}
</style>
