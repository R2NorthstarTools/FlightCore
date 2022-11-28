<script lang="ts">
import { defineComponent } from 'vue';
import { NorthstarState } from '../utils/NorthstarState';
import { ReleaseCanal } from '../utils/ReleaseCanal';

export default defineComponent({
    name: 'PlayButton',
    computed: {
        playButtonLabel(): string {
            if (this.$store.state.northstar_is_running) {
                return "Game is running";
            }

            switch(this.$store.state.northstar_state) {
                case NorthstarState.GAME_NOT_FOUND:
                    return "Select Titanfall2 game folder";
                case NorthstarState.INSTALL:
                    return "Install";
                case NorthstarState.INSTALLING:
                    return "Installing..."
                case NorthstarState.MUST_UPDATE:
                    return "Update";
                case NorthstarState.UPDATING:
                    return "Updating...";
                case NorthstarState.READY_TO_PLAY:
                    return "Launch game";

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
        selectOptions(): Object[] {
            return [
                {
                    label: 'In development',
                    options: [
                        {
                            value: ReleaseCanal.RELEASE_CANDIDATE,
                            label: 'Northstar development release',
                        },
                    ]
                },
                {
                    label: 'Live',
                    options: [
                        {
                            value: ReleaseCanal.RELEASE,
                            label: 'Northstar',
                        },
                    ]
                }
            ];
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
    <el-button :disabled="northstarIsRunning" type="primary" size="large" @click="launchGame" class="fc_launch__button">
        {{ playButtonLabel }}
    </el-button>
    <el-select v-model="value" placeholder="Select">
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
    border-radius: 2px;
    padding: 30px;
    font-size: 15px;
}
.fc_launch__button:focus {
    background-color: var(--el-color-primary);
    border-color: var(--el-color-primary);
}
</style>
