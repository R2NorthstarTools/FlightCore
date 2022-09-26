import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import {Tabs} from "../utils/Tabs";
import {invoke} from "@tauri-apps/api";
import {GameInstall} from "../utils/GameInstall";
import {ReleaseCanal} from "../utils/ReleaseCanal";

export const store = createStore({
    state () {
        return {
            current_tab: Tabs.PLAY,
            developer_mode: false,
            game_path: "this/is/the/game/path",

            installed_northstar_version: "Unknown version",

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

/**
 * This is called when application root component has been mounted.
 * It invokes all Rust methods that are needed to initialize UI.
 */
async function _initializeApp(state: any) {
    const result: GameInstall = await invoke("find_game_install_location_caller")
        .catch((err) => {
            // Gamepath not found or other error
            console.error(err);
            alert(err);
        });
    state.game_path = result.game_path;

    // Check installed Northstar version if found
    await _get_northstar_version_number_and_set_button_accordingly(state);
}

// TODO
async function _checkForFlightCoreUpdates(state: any) {
    // Get version number
    let version_number_string = await invoke("get_version_number") as string;
    // Check if up-to-date
    let flightcore_is_outdated = await invoke("check_is_flightcore_outdated_caller") as boolean;
}

/**
 * This registers callbacks listening to events from Rust-backend.
 * Those events include Origin and Northstar running state.
 */
function _initializeListeners(state: any) {
    listen("origin-running-ping", function (evt: TauriEvent<any>) {
        state.origin_is_running = evt.payload as boolean;
    });

    listen("northstar-running-ping", function (evt: TauriEvent<any>) {
        state.northstar_is_running = evt.payload as boolean;
    });
}

/**
 * This retrieves Northstar version tag, and stores it in application 
 * state, for it to be displayed in UI.
 */
async function _get_northstar_version_number_and_set_button_accordingly(state: any) {
    let northstar_version_number: string = await invoke("get_northstar_version_number_caller", { gamePath: state.game_path });
    if (northstar_version_number && northstar_version_number.length > 0) {
        state.installed_northstar_version = northstar_version_number;

        await invoke("check_is_northstar_outdated", { gamePath: state.game_path, northstarPackageName: ReleaseCanal.RELEASE })
            .then((message) => {
                console.log(message);
            })
            .catch((error) => {
                console.error(error);
                alert(error);
            });
    }
}
