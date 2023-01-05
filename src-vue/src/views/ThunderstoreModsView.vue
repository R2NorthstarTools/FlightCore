<template>
    <div class="fc-container" style="padding: 0">
        <div v-if="mods.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else class="container" ref="scrollbar">
            <div class="card-container">
                <div class="pagination_container">
                    <el-pagination
                        v-if="shouldDisplayPagination"
                        :currentPage="currentPageIndex + 1"
                        layout="prev, pager, next"
                        :page-size="modsPerPage"
                        :total="modsList.length"
                        @current-change="(e: number) => currentPageIndex = e - 1"
                    />
                </div>

                <!-- Message displayed if no mod matched searched words -->
                <div v-if="filteredMods.length === 0 && input.length !== 0 && !userIsTyping" class="modMessage">
                    No matching mod has been found.<br/>
                    Try another search!
                </div>

                <!-- Mod cards -->
                <thunderstore-mod-card v-for="mod of currentPageMods" v-bind:key="mod.name" :mod="mod" />
            </div>

            <!-- Bottom pagination -->
            <div class="card-container">
                <div class="pagination_container">
                    <el-pagination
                        class="fc_bottom__pagination"
                        v-if="shouldDisplayPagination"
                        :currentPage="currentPageIndex + 1"
                        layout="prev, pager, next"
                        :page-size="modsPerPage"
                        :total="modsList.length"
                        @current-change="onBottomPaginationChange"
                    />
                </div>
            </div>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { ThunderstoreMod } from "../utils/thunderstore/ThunderstoreMod";
import ThunderstoreModCard from "../components/ThunderstoreModCard.vue";
import { ElScrollbar, ScrollbarInstance } from "element-plus";

export default defineComponent({
    name: "ThunderstoreModsView",
    components: {ThunderstoreModCard},
    async mounted() {
        this.$store.commit('fetchThunderstoreMods');
    },
    props: {
        input: {
            required: true,
            type: String
        },
        searchValue: {
            required: true,
            type: String
        }
    },
    computed: {
        mods(): ThunderstoreMod[] {
            return this.$store.state.thunderstoreMods;
        },
        filteredMods(): ThunderstoreMod[] {
            if (this.searchValue.length === 0) {
                return this.mods;
            }

            return this.mods.filter((mod: ThunderstoreMod) => {
                return mod.name.toLowerCase().includes(this.searchValue)
                    || mod.owner.toLowerCase().includes(this.searchValue)
                    || mod.versions[0].description.toLowerCase().includes(this.searchValue);
            });
        },
        modsList(): ThunderstoreMod[] {
            return this.input.length !== 0 || this.userIsTyping ? this.filteredMods : this.mods;
        },
        modsPerPage(): number {
            return parseInt(this.$store.state.mods_per_page);
        },
        currentPageMods(): ThunderstoreMod[] {
            // User might want to display all mods on one page.
            const perPageValue = this.modsPerPage != 0 ? this.modsPerPage : this.modsList.length;

            const startIndex = this.currentPageIndex * perPageValue;
            const endIndexCandidate = startIndex + perPageValue;
            const endIndex =  endIndexCandidate > this.modsList.length ? this.modsList.length : endIndexCandidate;
            return this.modsList.slice(startIndex, endIndex);
        },
        shouldDisplayPagination(): boolean {
            return this.modsPerPage != 0 && this.modsList.length > this.modsPerPage;
        }
    },
    data() {
        return {
            modsBeingInstalled: [] as string[],
            userIsTyping: false,

            currentPageIndex: 0
        };
    },
    methods: {
        /**
         * This updates current pagination and scrolls view to the top.
         */
        onBottomPaginationChange(index: number) {
            this.currentPageIndex = index - 1;
            (this.$refs.scrollbar as ScrollbarInstance).scrollTo({ top: 0, behavior: 'smooth' });
        }
    },
    watch: {
        searchValue(_: string, __: string) {
            if (this.currentPageIndex !== 0) {
                this.currentPageIndex = 0;
            }
        }
    }
});
</script>

<style scoped>
.fc__changelog__container {
    padding: 20px 30px;
}

.fc-container:deep(.el-scrollbar__view) {
    padding-left: 0;
    padding-right: 0;
}

.el-timeline-item__timestamp {
    color: white !important;
    user-select: none !important;
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

.pagination_container {
    margin: 5px auto;
    padding: 0 5px;
    max-width: 1000px;
    justify-content: center;
    display: flex;
}

.el-pagination {
    margin: 0;
}

.fc_bottom__pagination {
    padding-bottom: 20px !important;
    padding-right: 10px;
}

/* Card container dynamic size */

.card-container {
    --thunderstore-mod-card-width: 178px;
    --thunderstore-mod-card-margin: 5px;
    --thunderstore-mod-card-columns-count: 1;

    width: calc(var(--thunderstore-mod-card-width) * var(--thunderstore-mod-card-columns-count) + var(--thunderstore-mod-card-margin) * 2 * var(--thunderstore-mod-card-columns-count));
}
@media (min-width: 628px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 2;
    }
}
@media (min-width: 836px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 3;
    }
}
@media (min-width: 1006px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 4;
    }
}
@media (min-width: 1196px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 5;
    }
}
@media (min-width: 1386px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 6;
    }
}
@media (min-width: 1576px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 7;
    }
}
@media (min-width: 1766px) {
    .card-container {
        --thunderstore-mod-card-columns-count: 8;
    }
}
</style>
