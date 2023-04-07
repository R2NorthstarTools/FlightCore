<template>
    <div class="fc-container">
        <el-scrollbar>
            <div class="fc_settings__container">
                <!-- Game folder location -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.manage_install') }}</h3>
                    <el-input
                        v-model="$store.state.game_path"
                        :placeholder="$t('settings.choose_folder')"
                        @click="updateGamePath"
                    >
                        <template #prepend>
                            <el-button icon="Folder" @click="updateGamePath"/>
                        </template>
                    </el-input>
                </div>

                <!-- Thunderstore mods per page configuration -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.nb_ts_mods_per_page') }}</h3>
                    <h6>
                        {{ $t('settings.nb_ts_mods_per_page_desc1') }}<br>
                        {{ $t('settings.nb_ts_mods_per_page_desc2') }}
                    </h6>
                    <el-input
                        v-model="modsPerPage"
                        type="number"
                    >
                        <template #append>
                            <el-button @click="modsPerPage = 20">
                                {{ $t('settings.nb_ts_mods_reset') }}
                            </el-button>
                        </template>
                    </el-input>
                </div>

                <!-- Interface localization -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.language') }}</h3>
                    <language-selector/>
                </div>

                <!-- Repair window -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.repair.title') }}</h3>
                    <el-button type="primary" @click="openRepairWindow">
                        {{ $t('settings.repair.open_window') }}
                    </el-button>
                </div>

                <!-- About section -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.about') }}</h3>
                    <div class="fc_northstar__version" @click="activateDeveloperMode">
                        {{ $t('settings.flightcore_version') }} {{ flightcoreVersion === '' ? 'Unknown version' : `${flightcoreVersion}` }}
                    </div>
                </div>

                <!-- Testing section -->
                <div class="fc_parameter__panel">
                    <h3>{{ $t('settings.testing') }}</h3>
                    <span>
                        {{ $t('settings.enable_test_channels') }}
                        <el-switch v-model="enableReleasesSwitch"></el-switch>
                    </span>
                </div>
            </div>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { Store } from 'tauri-plugin-store-api';
import { showNotification } from "../utils/ui";
import LanguageSelector from "../components/LanguageSelector.vue";
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: "SettingsView",
    components: {
        LanguageSelector
    },
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
            async set(value: boolean): Promise<void> {
                this.$store.state.enableReleasesSwitch = value;
                persistentStore.set('northstar-releases-switching', { value });
                await persistentStore.save(); // explicit save to disk

                // When disabling switch, we switch release canal to stable release, to avoid users being
                // stuck with release candidate after disabling release switching.
                if (!value && this.$store.state.northstar_release_canal !== ReleaseCanal.RELEASE) {
                    this.$store.commit('toggleReleaseCandidate');
                }
            }
        },
        modsPerPage: {
            get(): number {
                return this.$store.state.mods_per_page;
            },
            async set(value: number) {
                this.$store.state.mods_per_page = value;
                persistentStore.set('thunderstore-mods-per-page', { value });
                await persistentStore.save(); // explicit save to disk
            }
        }
    },
    methods: {
        activateDeveloperMode() {
            this.developerModeClicks += 1;
            if (this.developerModeClicks >= 6 && !this.$store.state.developer_mode) {
                this.$store.commit('toggleDeveloperMode');
                showNotification(
                    this.$t('settings.dev_mode_enabled_title'),
                    this.$t('settings.dev_mode_enabled_text'),
                    'info'
                );
                this.developerModeClicks = 0;
            }
        },
        async updateGamePath() {
            this.$store.commit('updateGamePath');
        },
        async openRepairWindow() {
            await invoke("open_repair_window")
                .then((message) => { })
                .catch((error) => {
                    showErrorNotification(error);
                });
        },
    },
    mounted() {
        document.querySelector('input')!.disabled = true;
    },
    unmounted() {
        if (('' + this.modsPerPage) === '') {
            console.warn('Incorrect value for modsPerPage, resetting it to 20.');
            this.modsPerPage = 20;
        }
    }
});
</script>

<style scoped>
.fc_settings__container {
    max-width: 1200px;
    margin: 0 auto;
}

h3:first-of-type {
    margin-top: 0;
    margin-bottom: 1em;
    text-transform: uppercase;
    font-weight: unset;
}

.el-input, .el-select {
    width: 50%;
}

.el-switch {
    margin-left: 50px;
}


/* Parameter panel styles */
.fc_parameter__panel {
    margin-bottom: 30px;
}

.fc_parameter__panel h3 {
    margin-bottom: 5px;
}

.fc_parameter__panel h6 {
    margin-top: 0;
    margin-bottom: 12px;
}
</style>
