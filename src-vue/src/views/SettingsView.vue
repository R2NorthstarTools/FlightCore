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
        <h3>About:</h3>
        <div class="fc_northstar__version" @click="activateDeveloperMode">
            FlightCore Version: {{ flightcoreVersion === '' ? 'Unknown version' : `${flightcoreVersion}` }}
        </div>
        <br />
        <br />
        UI design inspired by <el-link :underline="false" target="_blank" href="https://github.com/TFORevive/tforevive_launcher/" type="primary">TFORevive Launcher</el-link> (not yet public)

        <h3>Testing:</h3>
        <span>
            Enable testing release channels
            <el-switch v-model="enableReleasesSwitch"></el-switch>
        </span>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from 'element-plus';
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { Store } from 'tauri-plugin-store-api';
const persistentStore = new Store('flight-core-settings.json');

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
        enableReleasesSwitch: {
            get(): boolean {
                return this.$store.state.enableReleasesSwitch;
            },
            set(value: boolean): void {
                this.$store.state.enableReleasesSwitch = value;
                persistentStore.set('northstar-releases-switching', { value });

                // When disabling switch, we switch release canal to stable release, to avoid users being
                // stuck with release candidate after disabling release switching.
                if (!value && this.$store.state.northstar_release_canal !== ReleaseCanal.RELEASE) {
                    this.$store.commit('toggleReleaseCandidate');
                }
            }
        }
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

.el-switch {
    margin-left: 50px;
}
</style>
