<template>
    <el-dropdown trigger="click" placement="bottom-end" max-height="280" popper-class="fc_popper">
        <el-badge v-if="notifications.length != 0" :value="notifications.length" :max="9" class="item" type="primary">
            <el-button color="white" icon="BellFilled" circle />
        </el-badge>
        <el-button v-else color="white" icon="BellFilled" circle />
        <template #dropdown>
            <el-dropdown-menu>
                <el-alert
                    v-if="notifications.length != 0"
                    v-for="notification in notifications"
                    :key="JSON.stringify(notification)"
                    :title="notification.title"
                    :description="notification.text"
                    :type="notification.type"
                    show-icon
                    style="width: 300px"
                />
                <el-result
                    v-else
                    icon="success"
                    title="Up-to-date"
                    sub-title="Nothing to see here!"
                >
                    <template #icon>
                    </template>
                </el-result>
            </el-dropdown-menu>
        </template>
    </el-dropdown>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {Notification} from '../plugins/modules/notifications';

export default defineComponent({
    name: 'NotificationButton',
    computed: {
        notifications(): Notification[] {
            return this.$store.state.notifications.notifications;
        }
    },
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
