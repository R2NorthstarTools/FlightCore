<template>
    <el-dropdown trigger="click" placement="bottom-end" max-height="280" popper-class="fc_popper">
        <el-button color="white" icon="BellFilled" circle />
        <template #dropdown>
            <el-dropdown-menu>
                <el-alert
                    v-for="i in 10" :key="i"
                    title="success alert"
                    type="success"
                    description="more text description"
                    show-icon
                    style="width: 300px"
                />
            </el-dropdown-menu>
        </template>
    </el-dropdown>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { Store } from 'tauri-plugin-store-api';
import {Check, CircleCheck, CirclePlus, CirclePlusFilled, Plus} from "@element-plus/icons-vue";
const persistentStore = new Store('flight-core-settings.json');

export default defineComponent({
    name: 'NotificationButton',
    data: () => ({

    }),
    methods: {
        CircleCheck() {
            return CircleCheck
        },
        Check() {
            return Check
        },
        CirclePlus() {
            return CirclePlus
        },
        CirclePlusFilled() {
            return CirclePlusFilled
        },
        Plus() {
            return Plus
        }

    }
})
</script>

<style scoped>
.el-dropdown {
    height: 100%;
    max-height: 300px;
}

.el-button {
    margin: auto 25px auto 0 !important;
}

.el-alert {
    margin: 5px 10px 5px 5px;
}
</style>
