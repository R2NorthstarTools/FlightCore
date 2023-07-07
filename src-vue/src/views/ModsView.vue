<template>
    <div class="fc-container" style="display: flex">
        <!-- Local mods/Thunderstore mods menu -->
        <mods-menu
            :showingLocalMods="show_local_mods"
            @showLocalMods="(v) => show_local_mods = v"
        />

        <!-- Mods content -->
        <div class="fc_mods__container">
            <local-mods-view
                v-if="show_local_mods"
            />

            <thunderstore-mods-view
                v-else
                clearable
            />
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ThunderstoreModsView from "./mods/ThunderstoreModsView.vue";
import LocalModsView from "./mods/LocalModsView.vue";
import ModsMenu from "../components/ModsMenu.vue";

export default defineComponent({
    name: "ModsView",
    components: {
        ModsMenu,
        LocalModsView,
        ThunderstoreModsView
    },
    data() {
        return {
            show_local_mods: true,
        }
    },
    mounted() {
        // Fetch Thunderstore mods to eventually display outdated mods count
        this.$store.commit('fetchThunderstoreMods');
    }
});
</script>

<style scoped>
.fc_mods__container {
    display: flex;
    width: 100%;
    flex-direction: column;
}
</style>
