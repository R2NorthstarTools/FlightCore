import { ElNotification } from "element-plus";
import { invoke } from "@tauri-apps/api";
import { PullsApiResponseElement } from "../../../../src-tauri/bindings/PullsApiResponseElement";
import { PullRequestType } from '../../../../src-tauri/bindings/PullRequestType';

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
        }
    }
}
