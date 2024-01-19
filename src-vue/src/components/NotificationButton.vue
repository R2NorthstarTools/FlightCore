<template>
    <el-dropdown trigger="click" placement="bottom-end" max-height="280" popper-class="fc_popper">
        <el-badge v-if="notifications.length != 0" :value="notifications.length" :max="9" class="item" type="primary">
            <el-button color="white" icon="BellFilled" circle />
        </el-badge>
        <el-button v-else color="white" icon="BellFilled" circle />
        <template #dropdown>
            <el-dropdown-menu :key="counter">
                <el-alert
                    v-if="notifications.length != 0"
                    v-for="(notification, i) in notifications"
                    :key="i"
                    :title="notification.title"
                    :description="notification.text"
                    :type="notification.type"
                    show-icon
                    style="width: 300px"
                    @close.stop="removeNotification(i)"
                />
                <el-result
                    v-else
                    icon="success"
                    :title="i18n.global.tc('notification.no_new.title')"
                    :sub-title="i18n.global.tc('notification.no_new.text')"
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
import {i18n} from "../main";

export default defineComponent({
    name: 'NotificationButton',
    data: () => ({
        // This variable is used as a key for the dropdown menu, so we can force it to refresh when a item is deleted.
        counter: 0
    }),
    computed: {
        i18n() {
            return i18n
        },
        notifications(): Notification[] {
            return this.$store.state.notifications.notifications;
        }
    },
    methods: {
        removeNotification(index: number) {
            this.$store.commit('removeNotification', index);
            // By refreshing the notifications list, we ensure the first notification get the index 0, ensuring this list
            // is synchronized with the store list.
            this.counter += 1;
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

.el-badge:deep(sup) {
    transform: translate(-10px, 5px) !important;
}
</style>
