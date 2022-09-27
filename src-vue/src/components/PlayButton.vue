<script lang="ts">
import { NorthstarState } from '../utils/NorthstarState';

export default {
    name: 'PlayButton',
    data() {},
    computed: {
        playButtonLabel(): string {
            if (this.$store.state.northstar_is_running) {
                return "Game is running";
            }

            switch(this.$store.state.northstar_state) {
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
        }
    },
    methods: {
        launchGame() {
            this.$store.commit('launchGame');
        }
    }
};
</script>

<template>
    <el-button :disabled="northstarIsRunning" type="primary" size="large" @click="launchGame" class="fc_launch__button">
        {{ playButtonLabel }}
    </el-button>
</template>

<style scoped>
.fc_launch__button:focus {
    background-color: var(--el-color-primary);
    border-color: var(--el-color-primary);
}
</style>
