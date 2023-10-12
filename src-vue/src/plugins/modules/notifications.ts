type NotificationType = 'success' | 'warning' | 'info' | 'error';

export interface Notification {
    title: string;
    text: string;
    type: NotificationType;
}

interface NotificationsStoreState {
    notifications: Notification[];
}


export const notificationsModule = {
    state: () => ({
        notifications: []
    }) as NotificationsStoreState,
    mutations: {
        addNotification(state: NotificationsStoreState, payload: Notification) {
            state.notifications.push(payload);
        }
    }
  }
