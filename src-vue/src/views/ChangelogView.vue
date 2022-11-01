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
                <p v-html="formatRelease(release.body)"></p>
            </el-card>
            </el-timeline-item>
        </el-timeline>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { defineComponent } from 'vue';
import ReleaseInfo from '../utils/ReleaseInfo';
import { parse } from "marked";


export default defineComponent({
    name: "ChangelogView",
    data() {
        return {
            releases: [] as ReleaseInfo[]
        }
    },
    async mounted() {
        this.releases = await invoke("get_northstar_release_notes");
        console.log(this.releases);
    },
    methods: {
        // Transforms a Markdown document into an HTML document.
        // Taken from Viper launcher:
        // https://github.com/0neGal/viper/blob/5106d9ed409a3cc91a7755f961fab1bf91d8b7fb/src/app/launcher.js#L26
        formatRelease(releaseBody: string) {
            // GitHub authors' links formatting
            let content: string = releaseBody.replaceAll(/\@(\S+)/g, `<a target="_blank" href="https://github.com/$1">@$1</a>`);

            // PR's links formatting
            content = content.replaceAll(/\[(\S+)\]\(([^)]+)\)/g, `<a target="_blank" href="$2">$1</a>`);

            return parse(content, {breaks: true});
        }
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
