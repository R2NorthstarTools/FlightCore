import { PullsApiResponseElement } from "../../../../src-tauri/bindings/PullsApiResponseElement";

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
}
