import { ElNotification, NotificationHandle } from "element-plus";

/**
 * Displays content to the user in the form of a notification appearing on screen bottom right.
 **/
function showNotification(
    title: string,
    message: string = '',
    type: 'success' | 'warning' | 'error' | 'info' = 'success',
    duration: number = 4500
): NotificationHandle {
    return ElNotification({
        title, message, type, duration,
        position: 'bottom-right',
    });
}

export {showNotification};