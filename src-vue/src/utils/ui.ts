import { ElNotification, NotificationHandle } from "element-plus";

/**
 * Displays content to the user in the form of a notification appearing on screen bottom right.
 **/
function showNotification(
    title: string,
    message: string = '',
    type: 'success' | 'warning' | 'error' | 'info' = 'success'
): NotificationHandle {
    return ElNotification({
        title, message, type,
        position: 'bottom-right'
    });
}

export {showNotification};