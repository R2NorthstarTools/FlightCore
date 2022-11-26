<template>
    <div class="fc_repair__container">
        <h3>Repair:</h3>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ElNotification } from 'element-plus';

export default defineComponent({
    name: "Repair",
});
</script>

<style scoped>
.fc_repair__container {
    max-width: 1200px;
    padding: 20px 30px;
    margin: 0 auto;
    color: white;
    position: relative;
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
