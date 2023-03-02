import { ElNotification } from "element-plus";
import { invoke } from "@tauri-apps/api";
import { PullsApiResponseElement } from "../../../../src-tauri/bindings/PullsApiResponseElement";
import { PullRequestType } from '../../../../src-tauri/bindings/PullRequestType';
import { store } from "../store";

interface PullRequestStoreState {
    searchValue: string,
    pull_requests_launcher: PullsApiResponseElement[],
    pull_requests_mods: PullsApiResponseElement[],
}

export const pullRequestModule = {
    state: () => ({
        pull_requests_launcher: [],
        pull_requests_mods: [],
    }),
    mutations: {
        async getPullRequests(state: PullRequestStoreState, pull_request_type: PullRequestType) {
            await invoke<PullsApiResponseElement[]>("get_pull_requests_wrapper", { installType: pull_request_type })
                .then((message) => {
                    switch (pull_request_type) {
                        case "MODS":
                            state.pull_requests_mods = message;
                            break;

                        case "LAUNCHER":
                            state.pull_requests_launcher = message;
                            break;

                        default:
                            console.error("We should never end up here");
                    }
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                });
        },
        async installLauncherPR(state: PullRequestStoreState, pull_request: PullsApiResponseElement) {
            // Send notification telling the user to wait for the process to finish
            const notification = ElNotification({
                title: `Installing launcher PR ${pull_request.number}`,
                message: 'Please wait',
                duration: 0,
                type: 'info',
                position: 'bottom-right'
            });

            await invoke("apply_launcher_pr", { pullRequest: pull_request, gameInstallPath: store.state.game_path })
                .then((message) => {
                    console.log(message);
                    // Show user notification if mod install completed.
                    ElNotification({
                        title: `Done`,
                        message: `Installed ${pull_request.number}: "${pull_request.title}"`,
                        type: 'success',
                        position: 'bottom-right'
                    });
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                })
                .finally(() => {
                    // Clear old notification
                    notification.close();
                });
        },
        async installModsPR(state: PullRequestStoreState, pull_request: PullsApiResponseElement) {
            // Send notification telling the user to wait for the process to finish
            const notification = ElNotification({
                title: `Installing mods PR ${pull_request.number}`,
                message: 'Please wait',
                duration: 0,
                type: 'info',
                position: 'bottom-right'
            });

            await invoke("apply_mods_pr", { pullRequest: pull_request, gameInstallPath: store.state.game_path })
                .then((message) => {
                    // Show user notification if mod install completed.
                    ElNotification({
                        title: `Done`,
                        message: `Installed ${pull_request.number}: "${pull_request.title}"\nMake sure to launch via batch file or by specifying correct profile!`,
                        type: 'success',
                        position: 'bottom-right'
                    });
                })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                })
                .finally(() => {
                    // Clear old notification
                    notification.close();
                });
        },
    }
}
