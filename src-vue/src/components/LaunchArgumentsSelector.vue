<template>
    <div :class="containerClasses">
        <el-tooltip
            v-for="(argument, index) in arguments"
            class="box-item"
            :content="$t(argument.i18nEntry)"
            placement="bottom"
            :disabled="argument.i18nEntry.length === 0"
        >
            <el-check-tag
                class="fc-launch_arg_tag"
                :checked="values[index]"
                @change="onChange(index)"
            >
                {{ argument.argumentName }}
            </el-check-tag>
        </el-tooltip>

        <!-- Language selector -->
        <el-select v-if="displayLanguageSelector" v-model="langArgumentValue" class="m-2" placeholder="-lang=" @change="onLanguageSelection">
            <el-option
                v-for="item in langArgumentOptions"
                :key="item.value"
                :label="item.label"
                :value="item.value"
            />
        </el-select>

        <!-- User-input tag -->
        <el-input
            v-if="inputVisible"
            ref="InputRef"
            class="fc-tag__input"
            v-model="inputValue"
            size="small"
            @keyup.enter="handleInputConfirm"
            @blur="handleInputConfirm"
        />
        <el-button v-else class="button-new-tag ml-1" size="small" @click="showInput">
            {{ $t('settings.launch_args.new_arg_btn') }}
        </el-button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { LaunchArgument } from '../utils/LaunchArgument';
import { NorthstarState } from '../utils/NorthstarState';
import {invoke} from "@tauri-apps/api";
import { showErrorNotification } from '../utils/ui';

export default defineComponent({
    name: 'LaunchArgumentsSelector',
    computed: {
        arguments(): LaunchArgument[] {
            return (this.localCustomArgs.concat(this.officialArguments))
                .sort((a, b) => a.argumentName.localeCompare(b.argumentName));
        },
        displayLanguageSelector(): boolean {
            const langArgPrefix = '-language="';
            return this.arguments
                .map(arg => arg.argumentName)
                .filter(name => name.substring(0, langArgPrefix.length) === langArgPrefix)
                .length === 0;
        },
        officialArguments(): LaunchArgument[] {
            return [
                new LaunchArgument("-disablelogs", "settings.launch_args.descriptions.disablelogs"),
                new LaunchArgument("-vanilla", "settings.launch_args.descriptions.vanilla"),
                new LaunchArgument("-northstar", "settings.launch_args.descriptions.northstar"),
                new LaunchArgument("-dedicated", "settings.launch_args.descriptions.dedicated"),
                new LaunchArgument("-waitfordebugger", "settings.launch_args.descriptions.waitfordebugger"),
                new LaunchArgument("-enablechathooks", "settings.launch_args.descriptions.enablechathooks"),
                new LaunchArgument("-noplugins", "settings.launch_args.descriptions.noplugins"),
                new LaunchArgument("-novid", "settings.launch_args.descriptions.novid"),
                new LaunchArgument("-nosound", "settings.launch_args.descriptions.nosound")
            ];
        },
        containerClasses(): string {
            return this.gamePathIsSelected ? 'fc-tags_container' : 'fc-tags_container disabled_container';
        },
        gamePathIsSelected(): boolean {
            return this.$store.state.northstar_state !== NorthstarState.GAME_NOT_FOUND;
        },
        langArgumentOptions() {
            const languages = ['english', 'french', 'german', 'italian', 'japanese', 'mspanish', 'portuguese', 'russian', 'spanish', 'tchinese'];
            return languages.map(lang => ({value: lang, label: lang}));
        }
    },
    data: () => ({
        inputValue: '',
        inputVisible: false,

        langArgumentValue: '',
        values: [] as boolean[],
        localCustomArgs: [] as LaunchArgument[]
    }),
    methods: {
        onLanguageSelection( lang: string ) {
            this.createNewArgument( `-language="${lang}"` );
        },
        createNewArgument(arg: string) {
            let allArgumentsNames: string[] = this.arguments.map(arg => arg.argumentName);
            if (allArgumentsNames.indexOf(arg) !== -1) {
                console.warn(`Argument "${arg}" already present, ignoring.`);
            } else {
                const newArgument: LaunchArgument = new LaunchArgument(arg);
                this.localCustomArgs.push( newArgument );
                allArgumentsNames = this.arguments.map(arg => arg.argumentName);

                const index: number = allArgumentsNames.indexOf(newArgument.argumentName);
                this.values.splice(index, 0, true);
                this.saveLaunchArgumentsToFile();
            }
        },
        onChange(index: number) {
            this.values[index] = !this.values[index];
            this.saveLaunchArgumentsToFile();
        },
        saveLaunchArgumentsToFile() {
            const newArgs = this.arguments
                .filter((value: LaunchArgument, index: number) => {
                    return this.values[index];
                })
                .map((value: LaunchArgument) => value.argumentName)

            invoke<string[]>("set_launch_arguments", {
                gamePath: this.$store.state.game_path, arguments: newArgs
            }).catch((err: any) => {
                showErrorNotification(err);
            });
        },
        showInput() {
            this.inputVisible = true;
            this.$nextTick(() => {
                // @ts-ignore
                this.$refs.InputRef.input.focus();
            });
        },
        handleInputConfirm() {
            if (this.inputValue.length !== 0) {
                this.createNewArgument(this.inputValue);
            }
            this.inputVisible = false;
            this.inputValue = '';
        },
    },
    async mounted() {
        this.values = this.arguments.map(a => false);

        // Only add to local arguments those who are not in official arguments array
        const fileArgs = await invoke<string[]>("get_launch_arguments", { gamePath: this.$store.state.game_path});
        this.localCustomArgs = fileArgs
            .filter(arg => this.officialArguments.map(oArg => oArg.argumentName).indexOf(arg) === -1)
            .map(arg => new LaunchArgument(arg));

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

.fc-tag__input {
    width: auto;
}
</style>
