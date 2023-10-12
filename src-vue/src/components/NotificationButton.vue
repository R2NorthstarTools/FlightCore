<template>
    <el-button color="white" icon="BellFilled" circle />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { Store } from 'tauri-plugin-store-api';
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: 'NotificationButton',
    data: () => ({

    }),
    methods: {

    }
})
</script>

<style scoped>
.el-button {
    margin-right: 25px !important;
}
</style>
