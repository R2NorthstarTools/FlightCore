<template>
    <div style="height: calc(100% - var(--fc-menu_height))">
        <div v-if="mods.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else>
            <el-card v-for="mod of mods" v-bind:key="mod.name">
                <h1>{{ mod.name }}</h1>
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
                    versions: [
                        {
                            description: "Regular scorch is replaced by Shrek from the smash hit cult classic, Shrek",
                            icon: "https://gcdn.thunderstor…cons/UNO-Shrek-1.0.0.png",
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

    }
});
</script>

<style>
.el-scrollbar__view {
    padding: 20px 30px;
}

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

.el-card__body * {
    max-width: 100%;
}
</style>
