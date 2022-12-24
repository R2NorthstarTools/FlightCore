<template>
    <div class="fc-container">
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

                    <!-- Pagination -->
                    <el-pagination
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
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ThunderstoreMod } from "../utils/thunderstore/ThunderstoreMod";
import ThunderstoreModCard from "../components/ThunderstoreModCard.vue";

export default defineComponent({
    name: "ThunderstoreModsView",
    components: {ThunderstoreModCard},
    async mounted() {
        this.$store.commit('fetchThunderstoreMods');
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
        currentPageMods(): ThunderstoreMod[] {
            const startIndex = this.currentPageIndex * this.modsPerPage;
            const endIndexCandidate = startIndex + this.modsPerPage;
            const endIndex =  endIndexCandidate > this.modsList.length ? this.modsList.length : endIndexCandidate;
            return this.modsList.slice(startIndex, endIndex);
        }
    },
    data() {
        return {
            // This is the model for the search input.
            input: '',
            // This is the treated value of search input, updated every few milliseconds (debounced)
            searchValue: '',

            modsBeingInstalled: [] as string[],
            userIsTyping: false,
            debouncedSearch: this.debounce((i: string) => this.filterMods(i)),

            modsPerPage: 20,
            currentPageIndex: 0
        };
    },
    methods: {
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
         * triggered filtered mods recomputing by updating the `searchValue`
         * variable.
         *
         * This converts research string and all researched fields to
         * lower case, to match mods regardless of font case.
         */
        filterMods(value: string) {
            this.currentPageIndex = 0;
            this.searchValue = value.toLowerCase();
        },

        /**
         * This debounces a method, i.e. it prevents input method from being called
         * multiple times in a short period of time.
         * Stolen from https://www.freecodecamp.org/news/javascript-debounce-example/
         */
        debounce (func: Function, timeout = 200) {
            let timer: any;
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

    .el-pagination {
        float: none;
        margin-top: 5px;
    }
}

@media (max-width: 624px) {
    .card-container {
        width: 376px;
    }
}

.filter_container {
    margin-bottom: 10px;
}

.el-input {
    max-width: 200px;
}

@media (min-width: 812px) {
    .filter_container {
        margin: 5px auto;
        padding: 0 5px;
        max-width: 1000px;
    }

    .el-pagination {
        float: right;
        margin: 0;
    }
}

@media (min-width: 1000px) {
    .card-container {
        width: 940px;
    }

    .el-input {
        max-width: 300px;
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
