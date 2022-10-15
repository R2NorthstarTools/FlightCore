<template>
    <div class="fc__changelog__container">
        <el-timeline>
            <el-timeline-item
                v-for="release in releases"
                v-bind:key="release.name"
                :timestamp="release.published_at" 
                placement="top"
            >
            <el-card>
                <h4>{{ release.name }}</h4>
                {{ release.body }}
            </el-card>
            </el-timeline-item>
        </el-timeline>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { defineComponent } from 'vue';
import ReleaseInfo from '../utils/ReleaseInfo';

export default defineComponent({
    name: "ChangelogView",
    data() {
        return {
            releases: []
        }
    },
    async mounted() {
        this.releases = await invoke("get_northstar_release_notes");
        console.log(this.releases);
    }
});
</script>

<style scoped>
.fc__changelog__container {
    padding: 20px 30px;
}

.el-link {
  color: white;
}
</style>
