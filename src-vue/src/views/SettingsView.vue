<template>
    <div class="fc_settings__container">
        <!-- Game folder location -->
        <h3>Manage installation</h3>
        <el-input
            v-model="$store.state.game_path"
            class="w-50 m-2"
            placeholder="Choose installation folder"
            @click="updateGamePath"
        >
            <template #prepend>
                <el-button icon="Folder" @click="updateGamePath"/>
            </template>
        </el-input>
        <h3>About:</h3>
        UI design inspired by <a href="https://github.com/TFORevive/tforevive_launcher/">TFORevive Launcher</a> (not yet public)
    </div>
</template>

<script>
export default {
    name: "SettingsView",
    methods: {
        updateGamePath() {
            console.log('TODO: update path');
        }
    },
    mounted() {
        document.querySelector('input').disabled = true;
    }
}
</script>

<style scoped>
.fc_settings__container {
    max-width: 1200px;
    padding: 20px 30px;
    margin: 0 auto;
    color: white;
}

h3:first-of-type {
    margin-top: 0;
    margin-bottom: 1em;
    text-transform: uppercase;
    font-weight: unset;
}

.el-input {
    width: 50%;
}
</style>
