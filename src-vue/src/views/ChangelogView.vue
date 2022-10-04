<template>
    <div class="fc__changelog__container">
        <el-link :underline="false" icon="DocumentCopy" target="_blank" href="https://github.com/R2Northstar/Northstar/releases">
            Open release notes
        </el-link>
    </div>
</template>

<script lang="ts">
export default {
    name: "ChangelogView"
}
</script>

<style scoped>
.fc__changelog__container {
    padding: 20px 30px;
}

.el-link {
  color: white;
}
</style>
