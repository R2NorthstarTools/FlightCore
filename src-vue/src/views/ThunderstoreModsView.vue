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
                </div>

                <!-- Message displayed when user is typing in search bar -->
                <div v-if="userIsTyping" class="modMessage">
                    Searching mods...
                </div>

                <!-- Message displayed if no mod matched searched words -->
                <div v-else-if="filteredMods.length === 0 && input.length !== 0" class="modMessage">
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
                            <el-button type="primary" style="flex: 6">
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
import { shell } from '@tauri-apps/api';
import { ThunderstoreModVersion } from '../utils/thunderstore/ThunderstoreModVersion';

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
            return this.input.length === 0 ? this.mods : this.filteredMods;
        }
    },
    data() {
        return {
            input: '',
            filteredMods: [] as ThunderstoreMod[],
            userIsTyping: false,
            debouncedSearch: this.debounce((i: string) => this.filterMods(i))
        };
    },
    methods: {
        /**
         * Returns button text associated to a mod.
         * TODO Returned text changes regarding status of argument mod:
         *     * "Install", when the mod is not installed
         *     * "Update", when installed version is deprecated
         *     * "Installed", when mod is installed and up-to-date
         */
        getModButtonText(mod: ThunderstoreMod): string {
            return "Install";
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
        debounce (func: Function, timeout = 1000) {
            let timer: number;
            return (...args: any) => {
                this.userIsTyping = true;
                clearTimeout(timer);
                timer = setTimeout(() => {
                    this.userIsTyping = false;
                    func.apply(this, args);
                }, timeout);
            };
        }
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
