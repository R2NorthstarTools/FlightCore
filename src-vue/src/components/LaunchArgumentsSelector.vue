<template>
    <div>
        <el-check-tag 
            v-for="argument of arguments"
            checked
            class="fc-launch_arg_tag"
        >
            {{ argument.argumentName }}
        </el-check-tag>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { LaunchArgument } from '../utils/LaunchArgument';

export default defineComponent({
    name: 'LaunchArgumentsSelector',
    computed: {
        arguments(): LaunchArgument[] {
            return [
                new LaunchArgument("-disablelogs", "Disable logging and creation of log files"),
                new LaunchArgument("-vanilla", "Disables Northstar loading"),
                new LaunchArgument("-northstar", "Enables Northstar loading"),
                new LaunchArgument("-dedicated", "Starts a dedicated server without video output"),
                new LaunchArgument("-waitfordebugger", "Waits for debugger to connect before launching game"),
                new LaunchArgument("-enablechathooks", "Enables the use of chathooks for use by mods"),
                new LaunchArgument("-noplugins", "Disables the plugin system"),
                new LaunchArgument("-novid", "Disables startup videos"),
                new LaunchArgument("-nosound", "Disables all game sounds")
            ].sort((a, b) => a.argumentName.localeCompare(b.argumentName));
        }
    }
});
</script>

<style scoped>
.fc-launch_arg_tag {
    margin: 0 8px 8px 8px;
    white-space: nowrap;
}
</style>