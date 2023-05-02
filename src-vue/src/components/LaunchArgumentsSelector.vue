<template>
    <div :class="containerClasses">
        <el-tooltip
            v-for="(argument, index) in arguments"
            class="box-item"
            :content="argument.i18nEntry"
            placement="bottom"
        >
            <el-check-tag
                class="fc-launch_arg_tag"
                :checked="values[index]"
                @change="onChange(index)"
            >
                {{ argument.argumentName }}
            </el-check-tag>
        </el-tooltip>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { LaunchArgument } from '../utils/LaunchArgument';
import { NorthstarState } from '../utils/NorthstarState';
import {invoke} from "@tauri-apps/api";

export default defineComponent({
    name: 'LaunchArgumentsSelector',
    computed: {
        arguments(): LaunchArgument[] {
            const officialArguments = [
                new LaunchArgument("-disablelogs", "Disable logging and creation of log files"),
                new LaunchArgument("-vanilla", "Disables Northstar loading"),
                new LaunchArgument("-northstar", "Enables Northstar loading"),
                new LaunchArgument("-dedicated", "Starts a dedicated server without video output"),
                new LaunchArgument("-waitfordebugger", "Waits for debugger to connect before launching game"),
                new LaunchArgument("-enablechathooks", "Enables the use of chathooks for use by mods"),
                new LaunchArgument("-noplugins", "Disables the plugin system"),
                new LaunchArgument("-novid", "Disables startup videos"),
                new LaunchArgument("-nosound", "Disables all game sounds")
            ];

            return (this.localCustomArgs.concat(officialArguments))
                .sort((a, b) => a.argumentName.localeCompare(b.argumentName));
        },
        containerClasses(): string {
            return this.gamePathIsSelected ? 'fc-tags_container' : 'fc-tags_container disabled_container';
        },
        gamePathIsSelected(): boolean {
            return this.$store.state.northstar_state !== NorthstarState.GAME_NOT_FOUND;
        }
    },
    data: () => ({
        values: [] as boolean[],
        localCustomArgs: [] as LaunchArgument[]
    }),
    methods: {
        onChange(index: number) {
            this.values[index] = !this.values[index];

            const newArgs = this.arguments
                .filter((value: LaunchArgument, index: number) => {
                    return this.values[index];
                })
                .map((value: LaunchArgument) => value.argumentName)

            invoke<string[]>("set_launch_arguments", {
                gamePath: this.$store.state.game_path, arguments: newArgs
            });
        }
    },
    async mounted() {
        this.values = this.arguments.map(a => false);

        const fileArgs = await invoke<string[]>("get_launch_arguments", { gamePath: this.$store.state.game_path});
        this.localCustomArgs = fileArgs.map(arg => new LaunchArgument(arg, ''));
        
        this.arguments.forEach((argument, index) => {
            if (fileArgs.includes(argument.argumentName)) {
                this.values[index] = true;
            }
        });
    }
});
</script>

<style scoped>
.fc-launch_arg_tag {
    margin: 0 8px 8px 8px;
    white-space: nowrap;
}

.fc-tags_container {
    transform: translateX(-8px);
}

.disabled_container {
    pointer-events: none;
    filter: grayscale();
}
</style>
