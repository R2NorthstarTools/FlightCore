import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import {Tabs} from "../utils/Tabs";
import {invoke} from "@tauri-apps/api";
import {GameInstall} from "../utils/GameInstall";

export const store = createStore({
    state () {
        return {
            current_tab: Tabs.PLAY,
            developer_mode: false,
            game_path: "this/is/the/game/path",

            installed_northstar_version: "1.9.7",

            northstar_is_running: false,
            origin_is_running: false
        }
    },
    mutations: {
        toggleDeveloperMode(state) {
            state.developer_mode = !state.developer_mode;
        },
        initialize(state) {
            _initializeApp(state);
            // _checkForFlightCoreUpdates(state);
            _initializeListeners(state);
        },
        updateCurrentTab(state: any, newTab: Tabs) {
            state.current_tab = newTab;
        }
    }
});


async function _initializeApp(state: any) {
    const result: GameInstall = await invoke("find_game_install_location_caller")
        .catch((err) => {
            // Gamepath not found or other error
            console.error(err);
            alert(err);
        });
    state.game_path = result.game_path;
}

// TODO
async function _checkForFlightCoreUpdates(state: any) {
    // Get version number
    let version_number_string = await invoke("get_version_number") as string;
    // Check if up-to-date
    let flightcore_is_outdated = await invoke("check_is_flightcore_outdated_caller") as boolean;
}

function _initializeListeners(state: any) {
    listen("origin-running-ping", function (evt: TauriEvent<any>) {
        state.origin_is_running = evt.payload as boolean;
    });

    listen("northstar-running-ping", function (evt: TauriEvent<any>) {
        state.northstar_is_running = evt.payload as boolean;
    });
}
