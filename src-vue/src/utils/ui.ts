import { ElNotification, NotificationHandle } from "element-plus";
import { i18n } from "../main";

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

/**
 * Helper method displaying an error message to the user.
 **/
function showErrorNotification(
    error: string,
    title: string = i18n.global.tc('generic.error')
): NotificationHandle {
    return showNotification(title, error, 'error');
}

export {showNotification, showErrorNotification};
