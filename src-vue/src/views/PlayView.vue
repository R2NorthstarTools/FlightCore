<script setup lang="ts">
</script>

<template>
    <div class="fc_launch__container">
        <div class="fc_title">Northstar</div>
        <div class="fc_subtitle">Last updated: 23-09-2022 14:46</div>
        <el-button type="primary" size="large">Launch game</el-button>
        <el-button class="run__server" size="large">Run server</el-button>
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

.run__server {
    background-color: #6a6a6a75;
    border: none;
    color: white;
}

/* Titles */
.fc_title {
    color: white;
    font-size: 50px;
    font-weight: bold;
}

.fc_subtitle {
    color: rgb(168, 168, 168);
    margin-bottom: 20px;
}
</style>