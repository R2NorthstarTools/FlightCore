<template>
    <div class="fc-container">
        <div v-if="releases.length === 0" class="fc__changelog__container">
            <el-progress :show-text="false" :percentage="50" :indeterminate="true" />
        </div>
        <el-scrollbar v-else>
            <el-timeline>
                <el-timeline-item
                    v-for="release in releases"
                    v-bind:key="release.name"
                    :timestamp="formatDate(release.published_at)"
                    placement="top"
                >
                <el-card>
                    <h4>{{ release.name }}</h4>
                    <p v-html="formatRelease(release.body)"></p>
                </el-card>
                </el-timeline-item>
            </el-timeline>
        </el-scrollbar>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import ReleaseInfo from '../utils/ReleaseInfo';
import { marked } from "marked";


export default defineComponent({
    name: "ChangelogView",
    async mounted() {
        this.$store.commit('fetchReleaseNotes');
    },
    computed: {
        releases(): ReleaseInfo[] {
            return this.$store.state.releaseNotes;
        }
    },
    methods: {
        // Transforms a Markdown document into an HTML document.
        // Taken from Viper launcher:
        // https://github.com/0neGal/viper/blob/5106d9ed409a3cc91a7755f961fab1bf91d8b7fb/src/app/launcher.js#L26
        formatRelease(releaseBody: string) {
            // GitHub authors' links formatting
            let content: string = releaseBody.replaceAll(/\@(\S+)/g, `<a target="_blank" href="https://github.com/$1">@$1</a>`);

            // PR's links formatting
            content = content.replaceAll(/\[\#(\S+)\]\(([^)]+)\)/g, `<a target="_blank" href="$2">#$1</a>`);

            return marked.parse(content, {breaks: true});
        },
        // Formats an ISO-formatted date into a human-readable string.
        formatDate(timestamp: string): string {
            return new Date(timestamp).toLocaleDateString();
        }
    }
});
</script>

<style>
.el-scrollbar__view {
    padding: 20px 30px;
}

.fc__changelog__container {
    padding: 20px 30px;
}

.el-timeline-item__timestamp {
    color: white !important;
    user-select: none !important;
}

.el-card__body * {
    max-width: 100%;
}
</style>
