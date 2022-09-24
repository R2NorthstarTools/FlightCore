<script lang="ts">
import { ElNotification } from 'element-plus';
import {Tabs} from "../utils/Tabs";

export default {
    data() {
        return {
            developerModeClicks: 0
        }
    },
    computed: {
        northstarIsRunning(): boolean {
            return this.$store.state.northstar_is_running;
        }
    },
    methods: {
        activateDeveloperMode() {
            this.developerModeClicks += 1;
            if (this.developerModeClicks === 6) {
                this.$store.state.developer_mode = true;
                ElNotification({
                    title: 'Watch out!',
                    message: 'Developer mode enabled.',
                    type: 'info',
                    position: 'bottom-right'
                });
            }
        },

        showChangelogPage() {
            this.$store.commit('updateCurrentTab', Tabs.CHANGELOG);
        }
    }
};
</script>

<template>
    <div class="fc_launch__container">
        <div class="fc_title">Northstar</div>
        <div class="fc_northstar__version__container">
            <div class="fc_northstar__version" @click="activateDeveloperMode">
                v{{ $store.state.installed_northstar_version }}
            </div>
            <div class="fc_changelog__link" @click="showChangelogPage">
                (see patch notes)
            </div>
        </div>
        <div>
            <el-button :disabled="northstarIsRunning" type="primary" size="large" class="fc_launch__button">
                {{ northstarIsRunning ? "Game is running" : "Launch game" }}
            </el-button>
            <div v-if="$store.state.developer_mode" id="fc_services__status">
                <div>
                    <div class="fc_version__line">Northstar is running:    </div>
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

/* Buttons */
button {
    text-transform: uppercase;
    border-radius: 2px;
    padding: 30px;
    font-size: 15px;
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
