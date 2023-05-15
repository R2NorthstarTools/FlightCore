<template>
    <!-- Message displayed if no mod matched searched words -->
    <div v-if="mods.length === 0" class="noModMessage">
        {{ $t('mods.local.no_mods') }}
    </div>

    <el-scrollbar v-else>
    <local-mod-card v-for="mod of mods" v-bind:key="mod.name" :mod="mod" />
    </el-scrollbar>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { NorthstarMod } from "../../../../src-tauri/bindings/NorthstarMod";
import { fuzzy_filter } from "../../utils/filter";
import LocalModCard from "../../components/LocalModCard.vue";

export default defineComponent({
    name: 'LocalModsView',
    components: { LocalModCard },
    computed: {
        installedMods(): NorthstarMod[] {
            return this.$store.state.installed_mods;
        },
        searchValue(): string {
            return this.$store.getters.searchWords;
        },
        mods(): NorthstarMod[] {
            if (this.searchValue.length === 0) {
                return this.installedMods;
            }

            return this.installedMods.filter((mod: NorthstarMod) => {
                return fuzzy_filter(mod.name, this.searchValue);
            });
        }
    },
    data() {
        return {
            global_load_indicator: false,
        };
    },
    methods: {
    },
    mounted() {
        this.$store.commit('loadInstalledMods');
    }
})
</script>

<style scoped>

</style>
