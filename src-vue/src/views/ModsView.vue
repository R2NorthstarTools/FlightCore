<template>
    <div class="fc-container" style="display: flex">
        <!-- Local mods/Thunderstore mods menu -->
        <nav class="fc_mods__menu">
            <el-menu
                default-active="1"
                text-color="#fff"
            >
                <h5>Mods</h5>
                <el-menu-item index="1" @click="show_local_mods = true">
                    <el-icon><Folder /></el-icon>
                    <span>Local</span>
                </el-menu-item>
                <el-menu-item index="2" @click="show_local_mods = false">
                    <el-icon><Connection /></el-icon>
                    <span>Online</span>
                </el-menu-item>

                <!-- Search inputs -->
                <h5>Filter</h5>
                <el-input v-model="input" placeholder="Search" clearable @input="onFilterTextChange" />
                <el-select
                    v-if="!show_local_mods"
                    v-model="sortValue" 
                    placeholder="Sort mods"
                >
                    <el-option
                        v-for="item of sortValues"
                        :key="item.value"
                        :label="item.label"
                        :value="item.value"
                    />
                </el-select>
                <el-select
                    v-if="!show_local_mods"
                    v-model="modCategories"
                    multiple
                    placeholder="Select categories"
                >
                    <el-option
                        v-for="item in $store.state.thunderstoreModsCategories"
                        :key="item"
                        :label="item"
                        :value="item"
                    />
                </el-select>

            </el-menu>
        </nav>

        <!-- Mods content -->
        <div class="fc_mods__container">
            <local-mods-view
                v-if="show_local_mods"
            />

            <thunderstore-mods-view 
                v-else 
                :input="input"
                :searchValue="searchValue"
                :selectedCategories="modCategories"
                :modSorting="sortValue"
                clearable
            />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ThunderstoreModsView from "./ThunderstoreModsView.vue";
import LocalModsView from "./mods/LocalModsView.vue";
// @ts-ignore (why though?)
import { SortOptions } from "../utils/SortOptions.d.ts";

export default defineComponent({
    name: "ModsView",
    components: {
        LocalModsView,
        ThunderstoreModsView
    },
    data() {
        return {
            show_local_mods: true,

            // This is the model for the search input.
            input: '',
            // This is the treated value of search input
            searchValue: '',
            // Selected mod categories
            modCategories: [],
            sortValue: ''
        }
    },
    async mounted() {
        this.sortValue = this.sortValues[3].value;
    },
    computed: {
        sortValues(): {label: string, value: string}[] {
            return Object.keys(SortOptions).map((key: string) => ({
                value: key,
                label: SortOptions[key]
            }));
        }
    },
    methods: {
        /**
         * This method is called each time search input is modified, and
         * triggered filtered mods recomputing by updating the `searchValue`
         * variable.
         *
         * This converts research string and all researched fields to
         * lower case, to match mods regardless of font case.
         */
         onFilterTextChange(value: string) {
            this.searchValue = value.toLowerCase();
        },
    }
});
</script>

<style>
.fc_mods__menu {
    display: flex;
    max-width: 222px;
    min-width: 222px;
    padding: 10px;
}

.fc_mods__container {
    display: flex;
    width: 100%;
    flex-direction: column;
}

.fc_mods__menu h5 {
    margin: 8px 0 16px 5px;
}

.fc_mods__menu h5:not(:first-child){
    margin-top: 32px;
}

.fc_mods__menu > .el-menu {
    background-color: transparent;
    border: none;
    width: 100%;
}

.fc_mods__menu > .el-menu > .el-menu-item {
    height: 32px;
    margin-bottom: 5px;
    border-radius: 5px;
    color: #e2e6e7;
}

.fc_mods__menu > .el-menu > .el-menu-item:hover {
    background-color: #4e4e4e3b;
}

.fc_mods__menu > .el-menu > .el-menu-item.is-active {
    color: white;
    background-color: #4e4e4e7a;
}

.el-select {
    width: 100%;
    margin-top: 5px;
}
</style>
