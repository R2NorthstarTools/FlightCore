<template>
    <nav class="fc_mods__menu">
        <el-menu
            default-active="1"
            text-color="#fff"
        >
            <h5>Mods</h5>
            <el-menu-item index="1" @click="$emit('showLocalMods', true)">
                <el-icon><Folder /></el-icon>
                <span>{{ $t('mods.menu.local') }}</span>
            </el-menu-item>
            <el-menu-item index="2" @click="$emit('showLocalMods', false)">
                <el-icon><Connection /></el-icon>
                <span>{{ $t('mods.menu.online') }}</span>
                <el-badge :value="3" />
            </el-menu-item>

            <!-- Search inputs -->
            <h5>{{ $t('mods.menu.filter') }}</h5>
            <el-input v-model="$store.state.search.searchValue" :placeholder="$t('mods.menu.search')" clearable />
            <el-select
                v-if="!showingLocalMods"
                v-model="$store.state.search.sortValue" 
                :placeholder="$t('mods.menu.sort_mods')"
            >
                <el-option
                    v-for="item of sortValues"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                />
            </el-select>
            <el-select
                v-if="!showingLocalMods"
                v-model="$store.state.search.selectedCategories"
                multiple
                :placeholder="$t('mods.menu.select_categories')"
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
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { SortOptions } from '../utils/SortOptions.d';

export default defineComponent({
    name: 'ModsMenu',
    props: {
        showingLocalMods: {
            required: true,
            type: Boolean
        }
    },
    mounted() {
        this.$store.state.search.sortValue = this.sortValues[3].value;
    },
    computed: {
        sortValues(): {label: string, value: string}[] {
            return Object.keys(SortOptions).map((key: string) => ({
                value: key,
                label: this.$t('mods.menu.sort.' + Object.values(SortOptions)[Object.keys(SortOptions).indexOf(key)])
            }));
        }
    }
})
</script>

<style scoped>
.fc_mods__menu {
    display: flex;
    max-width: 222px;
    min-width: 222px;
    padding: 10px;
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
