<template>
    <div class="fc__developer__container">
        <el-button type="primary" @click="disableDevMode">
            Disable developer mode
        </el-button>
    </div>
</template>

<script lang="ts">
export default {
    name: "DeveloperView",
    methods: {
        disableDevMode() {
            this.$store.commit('toggleDeveloperMode');
        }
    }
}
</script>

<style scoped>
.fc__developer__container {
    padding: 20px 30px;
}
</style>
