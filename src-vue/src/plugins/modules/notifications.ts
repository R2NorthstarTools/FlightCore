type NotificationType = 'success' | 'warning' | 'info' | 'error';

export interface Notification {
    title: string;
    text: string;
    type: NotificationType;
}

interface NotificationsStoreState {
    notifications: Notification[];
}


/**
 * This notification module is meant to host the list of notifications that have been fired while the application was
 * not focused.
 * This list is then used by the [NotificationButton] component to display notifications to user.
 **/
export const notificationsModule = {
    state: () => ({
        notifications: []
    }) as NotificationsStoreState,
    mutations: {
        addNotification(state: NotificationsStoreState, payload: Notification) {
            state.notifications.push(payload);
        },
        removeNotification(state: NotificationsStoreState, index: number): void {
            state.notifications.splice(index, 1);
        }
    }
  }
