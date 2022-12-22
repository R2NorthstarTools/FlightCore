<script lang="ts">
import { ElNotification } from 'element-plus';
import { Tabs } from "../utils/Tabs";
import PlayButton from '../components/PlayButton.vue';
import { defineComponent } from "vue";

export default defineComponent({
    components: {
        PlayButton
    },
    computed: {
        northstarIsRunning(): boolean {
            return this.$store.state.northstar_is_running;
        },
        northstarVersion(): string {
            return this.$store.state.installed_northstar_version;
        },
    },
    methods: {
        showChangelogPage() {
            this.$store.state.changelog_view_visible = true;
            this.$store.commit('updateCurrentTab', Tabs.CHANGELOG);
        }
    }
});
</script>

<template>
    <div class="fc_launch__container">
        <div class="fc_title">Northstar</div>
        <div class="fc_northstar__version__container">
            {{ northstarVersion === '' ? 'Unknown version' : `v${northstarVersion}` }}
            <div v-if="northstarVersion !== ''" class="fc_changelog__link" @click="showChangelogPage">
                (see patch notes)
            </div>
        </div>
        <div>
            <PlayButton />
            <div v-if="$store.state.developer_mode" id="fc_services__status">
                <div>
                    <div class="fc_version__line">Northstar is running: </div>
                    <div class="fc_version__line fc_version__line__boolean"> {{ northstarIsRunning }}</div>
                </div>
                <div>
                    <div class="fc_version__line">Origin is running: </div>
                    <div class="fc_version__line fc_version__line__boolean">{{ $store.state.origin_is_running }}</div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.fc_launch__container {
    margin: 50px;
    position: fixed;
    bottom: 0;
}

/* Titles */
.fc_title {
    color: white;
    font-size: 50px;
    font-weight: bold;
}

/* Northstar version + changelog link */
.fc_northstar__version__container {
    margin-bottom: 20px;
    color: rgb(168, 168, 168);
}

.fc_northstar__version, .fc_changelog__link {
    display: inline-block;
}

.fc_changelog__link {
    margin-left: 3px;
    text-decoration: underline;
    cursor: pointer;
}


.fc_launch__button:focus {
    background-color: var(--el-color-primary);
    border-color: var(--el-color-primary);
}


#fc_services__status {
    display: inline-block;
    position: fixed;
    padding: 10px 20px;
    color: #e8edef;
    bottom: 43px;
}

.fc_version__line {
    display: inline-block;
}

.fc_version__line__boolean {
    margin-left: 5px;
    margin-bottom: 5px;
    color: #b4b6b9;
}
</style>
