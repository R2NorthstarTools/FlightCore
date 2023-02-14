<script lang="ts">
import { defineComponent } from 'vue';
import { NorthstarState } from '../utils/NorthstarState';
import { ReleaseCanal } from '../utils/ReleaseCanal';

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
                            label: 'Northstar release candidate',
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
        }
    },
    methods: {
        launchGame() {
            this.$store.commit('launchGame');
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
</template>

<style scoped>

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
