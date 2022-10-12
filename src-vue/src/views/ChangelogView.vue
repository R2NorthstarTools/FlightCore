<template>
    <div class="fc__changelog__container">
        <el-link :underline="false" icon="DocumentCopy" target="_blank" href="https://github.com/R2Northstar/Northstar/releases">
            Open release notes
        </el-link>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { defineComponent } from 'vue';
import ReleaseInfo from '../utils/ReleaseInfo';

export default defineComponent({
    name: "ChangelogView",
    mounted: async () => {
        const notes: ReleaseInfo[] = await invoke("get_northstar_release_notes");
        console.log(notes);
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
