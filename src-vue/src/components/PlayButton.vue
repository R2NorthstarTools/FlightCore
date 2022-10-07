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
                    return "Select game folder";
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
