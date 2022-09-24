<script lang="ts">
import { ElNotification } from 'element-plus';

export default {
    data() {
        return {
            developerModeClicks: 0
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
        }
    }
};
</script>

<template>
    <div class="fc_launch__container">
        <div class="fc_title">Northstar</div>
        <div class="fc_northstar__version" @click="activateDeveloperMode">
            v{{ $store.state.installed_northstar_version }}
        </div>
        <div>
            <el-button type="primary" size="large">Launch game</el-button>
            <div v-if="$store.state.developer_mode" id="fc_services__status">
                <div>
                    <div class="fc_version__line">Northstar is running:    </div>
                    <div class="fc_version__line fc_version__line__boolean"> {{ $store.state.northstar_is_running }}</div>
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

.fc_northstar__version {
    color: rgb(168, 168, 168);
    margin-bottom: 20px;
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
