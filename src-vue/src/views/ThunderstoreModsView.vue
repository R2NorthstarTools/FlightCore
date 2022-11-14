<template>
    <div style="height: calc(100% - var(--fc-menu_height))">
        <div v-if="mods.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else class="container">
            <!-- Search filters -->
            <div class="filter_container">
                <el-input v-model="input" placeholder="Search" clearable @input="onFilterTextChange" />
            </div>

            <!-- Message displayed if no mod matched searched words -->
            <div v-if="filteredMods.length === 0 && input.length !== 0" class="noModFound">
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
    /**
     * TODO Mods should be stored in front store
     * TODO Fetch mods with backend?
     */
    async mounted() {
        const response = await fetch('https://northstar.thunderstore.io/api/v1/package/');
        this.mods = JSON.parse(await (await response.blob()).text());

        // Remove some mods from listing
        const removedMods = ['Northstar', 'NorthstarReleaseCandidate', 'r2modman'];
        this.mods = this.mods.filter((mod: ThunderstoreMod) => !removedMods.includes(mod.name));
    },
    computed: {
        modsList(): ThunderstoreMod[] {
            return this.input.length === 0 ? this.mods : this.filteredMods;
        }
    },
    data() {
        return {
            input: '',
            mods: [] as ThunderstoreMod[],
            filteredMods: [] as ThunderstoreMod[]
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
         * This method is called each time search input is modified, and
         * filters mods matching the input string.
         * 
         * This converts research string and all researched fields to
         * lower case, to match mods regardless of font case.
         */
        onFilterTextChange(value: string) {
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
        openURL(url: string): void {
            shell.open(url);
        },
        modDownloadsCount(mod: ThunderstoreMod): number {
            let totalDownloads = 0;
            mod.versions.map((version: ThunderstoreModVersion) => totalDownloads += version.downloads);
            return totalDownloads;
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

.noModFound {
    color: white;
    margin: 20px 5px;
}
</style>
