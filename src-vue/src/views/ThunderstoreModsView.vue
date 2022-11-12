<template>
    <div style="height: calc(100% - var(--fc-menu_height))">
        <div v-if="mods.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else class="container">
            <el-card v-for="mod of mods" v-bind:key="mod.name" :body-style="{ padding: '0px' }">
                <img
                    :src="mod.versions[0].icon"
                    class="image"
                />
                <div style="padding: 14px 14px 10px 14px;">
                    <span>{{ mod.name }}</span><br/>
                    <span class="author">by {{ mod.owner }}</span>
                    <div class="desc">
                        {{ mod.versions[0].description }}
                    </div>
                    <el-button type="primary" class="button">
                        {{ getModButtonText(mod) }}
                    </el-button>
                </div>
            </el-card>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {ThunderstoreMod} from "../utils/thunderstore/ThunderstoreMod";

export default defineComponent({
    name: "ThunderstoreModsView",
    async mounted() {

    },
    computed: {
        mods(): ThunderstoreMod[] {
            return [
                {
                    name: "Shrek",
                    owner: "UNO",
                    versions: [
                        {
                            description: "Regular scorch is replaced by Shrek from the smash hit cult classic, Shrek",
                            icon: "https://gcdn.thunderstore.io/live/repository/icons/UNO-Shrek-1.0.0.png",
                            version_number: "1.0.0",
                            download_url: "https://thunderstore.io/package/download/UNO/Shrek/1.0.0/",
                            date_created: "2022-11-10T00:03:13.057122Z"
                        }
                    ]
                }
            ];
        }
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
    max-width: 185px;
}

.author {
    font-size: 14px;
    font-style: italic;
}

.desc {
    font-size: 12px;
    margin: 8px 0 16px;
}

button {
    width: 100% !important;
}
</style>
