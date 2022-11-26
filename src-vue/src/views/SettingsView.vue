<template>
    <div class="fc_settings__container">
        <!-- Game folder location -->
        <h3>Manage installation</h3>
        <el-input
            v-model="$store.state.game_path"
            class="w-50 m-2"
            placeholder="Choose installation folder"
            @click="updateGamePath"
        >
            <template #prepend>
                <el-button icon="Folder" @click="updateGamePath"/>
            </template>
        </el-input>

        <h3>Other:</h3>

        <el-button type="primary" @click="openRepairView">
            Open Repair options
        </el-button>

        <h3>About:</h3>
        <div class="fc_northstar__version" @click="activateDeveloperMode">
            FlightCore Version: {{ flightcoreVersion === '' ? 'Unknown version' : `${flightcoreVersion}` }}
        </div>
        <br />
        <br />
        UI design inspired by <el-link :underline="false" target="_blank" href="https://github.com/TFORevive/tforevive_launcher/" type="primary">TFORevive Launcher</el-link> (not yet public)
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from 'element-plus';
import { Tabs } from "../utils/Tabs";

export default defineComponent({
    name: "SettingsView",
    data() {
        return {
            developerModeClicks: 0
        }
    },
    computed: {
        flightcoreVersion(): string {
            return this.$store.state.flightcore_version;
        },
    },
    methods: {
        activateDeveloperMode() {
            this.developerModeClicks += 1;
            if (this.developerModeClicks >= 6) {
                this.$store.state.developer_mode = true;
                ElNotification({
                    title: 'Watch out!',
                    message: 'Developer mode enabled.',
                    type: 'info',
                    position: 'bottom-right'
                });
                this.developerModeClicks = 0;
            }
        },
        async updateGamePath() {
            this.$store.commit('updateGamePath');
        },
        async openRepairView() {
            this.$store.state.repair_view_visible = true;
            this.$store.commit('updateCurrentTab', Tabs.REPAIR);
        }
    },
    mounted() {
        document.querySelector('input')!.disabled = true;
    }
});
</script>

<style scoped>
.fc_settings__container {
    max-width: 1200px;
    padding: 20px 30px;
    margin: 0 auto;
    color: white;
    position: relative;
}

h3:first-of-type {
    margin-top: 0;
    margin-bottom: 1em;
    text-transform: uppercase;
    font-weight: unset;
}

.el-input {
    width: 50%;
}
</style>
