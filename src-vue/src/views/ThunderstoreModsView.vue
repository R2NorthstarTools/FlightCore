<template>
    <div style="height: calc(100% - var(--fc-menu_height))">
        <div v-if="mods.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else class="container">
            <div class="card-container">
                <!-- Search filters -->
                <div class="filter_container">
                    <el-input v-model="input" placeholder="Search" clearable @input="onFilterTextChange" />
                    <!-- Message displayed when user is typing in search bar -->
                    <div v-if="userIsTyping" class="modMessage search">
                        Searching mods...
                    </div>
                </div>

                <!-- Message displayed if no mod matched searched words -->
                <div v-if="filteredMods.length === 0 && input.length !== 0 && !userIsTyping" class="modMessage">
                    No matching mod has been found.<br/>
                    Try another search!
                </div>

                <!-- Mod cards -->
                <el-card v-for="mod of modsList" v-bind:key="mod.name" :body-style="{ padding: '0px' }">
                    <img
                        :src="mod.versions[0].icon"
                        class="image"
                    />
                    <div style="padding: 0px 10px 10px;">
                        <span class="statContainer">
                            <el-icon class="no-inherit">
                                <Download />
                            </el-icon>
                            {{ modDownloadsCount(mod) }}
                        </span>

                        <span class="statContainer">
                            {{ mod.rating_score }}
                            <el-icon class="no-inherit">
                                <Star />
                            </el-icon>
                        </span>
                        <br/>

                        <div class="name hide-text-overflow">{{ mod.name }}</div>
                        <div class="author hide-text-overflow">by {{ mod.owner }}</div>
                        <div class="desc">
                            {{ mod.versions[0].description }}
                        </div>

                        <span style="display: flex">
                            <el-button
                                :type="getModButtonType(mod)"
                                style="flex: 6"
                                :loading="modsBeingInstalled.includes(mod.name)"
                                @click.stop="installMod(mod)"
                            >
                                {{ getModButtonText(mod) }}
                            </el-button>
                            <el-button link type="info" class="infoBtn" @click="openURL(mod.package_url)">
                                <el-icon>
                                    <InfoFilled />
                                </el-icon>
                            </el-button>
                        </span>
                    </div>
                </el-card>
            </div>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {ThunderstoreMod} from "../utils/thunderstore/ThunderstoreMod";
import {invoke, shell} from '@tauri-apps/api';
import {ThunderstoreModVersion} from '../utils/thunderstore/ThunderstoreModVersion';
import {GameInstall} from "../utils/GameInstall";
import {ElNotification} from "element-plus";
import {NorthstarMod} from "../utils/NorthstarMod";
import {ThunderstoreModStatus} from "../utils/thunderstore/ThunderstoreModStatus";

export default defineComponent({
    name: "ThunderstoreModsView",
    async mounted() {
        this.$store.commit('fetchThunderstoreMods');
    },
    computed: {
        mods(): ThunderstoreMod[] {
            return this.$store.state.thunderstoreMods;
        },
        modsList(): ThunderstoreMod[] {
            return this.input.length === 0 || this.userIsTyping ? this.mods : this.filteredMods;
        }
    },
    data() {
        return {
            input: '',
            filteredMods: [] as ThunderstoreMod[],
            modsBeingInstalled: [] as string[],
            userIsTyping: false,
            debouncedSearch: this.debounce((i: string) => this.filterMods(i))
        };
    },
    methods: {
        /**
         * Returns button type associated to a mod.
         */
        getModButtonType(mod: ThunderstoreMod): string {
            switch (this.getModStatus(mod)) {
                case ThunderstoreModStatus.BEING_INSTALLED:
                    return "primary";
                case ThunderstoreModStatus.INSTALLED:
                    return "success";
                case ThunderstoreModStatus.NOT_INSTALLED:
                    return "primary";
                case ThunderstoreModStatus.OUTDATED:
                    return "warning";
            }
        },

        /**
         * Returns button text associated to a mod.
         */
        getModButtonText(mod: ThunderstoreMod): string {
            switch (this.getModStatus(mod)) {
                case ThunderstoreModStatus.BEING_INSTALLED:
                    return "Installing...";
                case ThunderstoreModStatus.INSTALLED:
                    return "Installed";
                case ThunderstoreModStatus.NOT_INSTALLED:
                    return "Install";
                case ThunderstoreModStatus.OUTDATED:
                    return "Update";
            }
        },

        /**
         * Returns the status of a given mod.
         * TODO Returned status changes regarding status of argument mod:
         *     * "Outdated", when installed version is deprecated
         *     * "Installed", when mod is installed and up-to-date
         */
        getModStatus(mod: ThunderstoreMod): ThunderstoreModStatus {
            if (this.modsBeingInstalled.includes(mod.name)) {
                return ThunderstoreModStatus.BEING_INSTALLED;
            }
            // TODO ensure mod is up-to-date
            if (this.$store.state.installed_mods.map((mod: NorthstarMod) => mod.thunderstore_mod_string).includes(mod.versions[0].full_name)) {
                return ThunderstoreModStatus.INSTALLED;
            }
            return ThunderstoreModStatus.NOT_INSTALLED;
        },

        /**
         * This is a debounced version of the filterMods method, that calls
         * filterMods when user has stopped typing in the search bar (i.e.
         * waits 300ms).
         * It allows not to trigger filtering method (which is costly) each
         * time user inputs a character.
         */
        onFilterTextChange (searchString: string) {
            this.debouncedSearch(searchString);
        },

        /**
         * This method is called each time search input is modified, and
         * filters mods matching the input string.
         *
         * This converts research string and all researched fields to
         * lower case, to match mods regardless of font case.
         */
        filterMods(value: string) {
            if (value === '') {
                this.filteredMods = [];
                return;
            }

            const searchValue = value.toLowerCase();

            this.filteredMods = this.mods.filter((mod: ThunderstoreMod) => {
                return mod.name.toLowerCase().includes(searchValue)
                    || mod.owner.toLowerCase().includes(searchValue)
                    || mod.versions[0].description.toLowerCase().includes(searchValue);
            });
        },

        /**
         * This opens an URL in user's favorite web browser.
         * This is used to open Thunderstore mod pages.
         */
        openURL(url: string): void {
            shell.open(url);
        },

        /**
         * This computes the total count of downloads of a given mod, by adding
         * download count of each of its releases.
         */
        modDownloadsCount(mod: ThunderstoreMod): number {
            let totalDownloads = 0;
            mod.versions.map((version: ThunderstoreModVersion) => totalDownloads += version.downloads);
            return totalDownloads;
        },

        /**
         * This debounces a method, i.e. it prevents input method from being called
         * multiple times in a short period of time.
         * Stolen from https://www.freecodecamp.org/news/javascript-debounce-example/
         */
        debounce (func: Function, timeout = 200) {
            let timer: number;
            return (...args: any) => {
                this.userIsTyping = true;
                clearTimeout(timer);
                timer = setTimeout(() => {
                    this.userIsTyping = false;
                    func.apply(this, args);
                }, timeout);
            };
        },

        async installMod (mod: ThunderstoreMod) {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;
            this.modsBeingInstalled.push(mod.name);
            await invoke("install_mod_caller", { gameInstall: game_install, thunderstoreModString: mod.versions[0].full_name }).then((message) => {
                ElNotification({
                    title: `Installed ${mod.name}`,
                    message: message as string,
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
                this.modsBeingInstalled.splice(this.modsBeingInstalled.indexOf(mod.name), 1);
            });
        },
    }
});
</script>

<style scoped>
.fc__changelog__container {
    padding: 20px 30px;
    position: relative;
    overflow-y: auto;
    height: calc(100% - var(--fc-menu_height));
    color: white;
}

.el-timeline-item__timestamp {
    color: white !important;
    user-select: none !important;
}

.el-card {
    display: inline-block;
    max-width: 178px;
    margin: 5px;
}

.author {
    font-size: 14px;
    font-style: italic;
}

.hide-text-overflow {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
}

.desc {
    font-size: 12px;
    margin: 8px 0 16px;
    height: 57px;
    text-overflow: ellipsis;
    overflow: hidden;
}

.statContainer {
    font-size: 14px;
}

.statContainer:nth-child(2) {
    float: right;
}

.filter_container {
    margin: 5px;
}

.el-input {
    max-width: 300px;
}

.infoBtn {
    width: 20px;
    padding: 0;
    font-size: 20px;
    border: none;
}

.search {
    display: inline-block;
    margin: 0 0 0 10px !important;
}

.modMessage {
    color: white;
    margin: 20px 5px;
}

.card-container {
    margin: 0 auto;
}

/* Card container dynamic size */
@media (max-width: 1000px) {
    .card-container {
        width: 752px;
    }
}

@media (max-width: 812px) {
    .card-container {
        width: 574px;
    }
}

@media (max-width: 624px) {
    .card-container {
        width: 376px;
    }
}

@media (min-width: 1000px) {
    .card-container {
        width: 940px;
    }
}

@media (min-width: 1188px) {
    .card-container {
        width: 1128px;
    }
}

@media (min-width: 1376px) {
    .card-container {
        width: 1316px;
    }
}

@media (min-width: 1565px) {
    .card-container {
        width: 1505px;
    }
}
</style>
