<template>
    <el-select v-model="value" class="m-2" 
        :placeholder="$t('settings.language_select')" size="large">
        <el-option
            v-for="item in options"
            :key="item.value"
            :label="item.label"
            :value="item.value"
        />
  </el-select>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'LanguageSelector',
    data: () => ({
        value: '',
        options: [
            {
                value: 'en',
                label: 'English'
            },
            {
                value: 'fr',
                label: 'Français'
            }
        ]
    }),
})
</script>
