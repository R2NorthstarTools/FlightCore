import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { Tabs } from "../utils/Tabs";
import { InstallType } from "../utils/InstallType";
import { invoke } from "@tauri-apps/api";
import { GameInstall } from "../utils/GameInstall";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { ElNotification, NotificationHandle } from 'element-plus';
import { NorthstarState } from '../utils/NorthstarState';
import { appDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';
import { Store } from 'tauri-plugin-store-api';
import {router} from "../main";

const persistentStore = new Store('flight-core-settings.json');


export interface FlightCoreStore {
    developer_mode: boolean,
    game_path: string,
    install_type: InstallType,

    flightcore_version: string,

    installed_northstar_version: string,
    northstar_state: NorthstarState,
    northstar_release_canal: ReleaseCanal,

    northstar_is_running: boolean,
    origin_is_running: boolean
}

let notification_handle: NotificationHandle;

export const store = createStore<FlightCoreStore>({
    state (): FlightCoreStore {
        return {
            developer_mode: false,
            game_path: undefined as unknown as string,
            install_type: undefined as unknown as InstallType,

            flightcore_version: "",

            installed_northstar_version: "",
            northstar_state: NorthstarState.GAME_NOT_FOUND,
            northstar_release_canal: ReleaseCanal.RELEASE,

            northstar_is_running: false,
            origin_is_running: false
        }
    },
    mutations: {
        checkNorthstarUpdates(state) {
            _get_northstar_version_number(state);
        },
        toggleDeveloperMode(state) {
            state.developer_mode = !state.developer_mode;

            // Reset tab when closing dev mode.
            if (!state.developer_mode) {
                store.commit('updateCurrentTab', Tabs.PLAY);
            }
        },
        initialize(state) {
            _initializeApp(state);
            _checkForFlightCoreUpdates(state);
            _initializeListeners(state);
        },
        updateCurrentTab(state: any, newTab: Tabs) {
            router.push({path: newTab});
        },
        async updateGamePath(state: FlightCoreStore) {
            // Open a selection dialog for directories
            const selected = await open({
                directory: true,
                multiple: false,
                defaultPath: await appDir(),
            });
            if (Array.isArray(selected)) {
                // user selected multiple directories
                alert("Please only select a single directory");
            } else if (selected === null) {
                // user cancelled the selection
            } else {
                // user selected a single directory

                // Verify if valid Titanfall2 install location
                let is_valid_titanfall2_install = await invoke("verify_install_location", { gamePath: selected }) as boolean;
                if (is_valid_titanfall2_install) {
                    state.game_path = selected;
                    ElNotification({
                        title: 'New game folder',
                        message: "Game folder was successfully updated.",
                        type: 'success',
                        position: 'bottom-right'
                    });
                    notification_handle.close();
                    state.install_type = InstallType.UNKNOWN;

                    // Check for Northstar install
                    store.commit('checkNorthstarUpdates');
                }
                else {
                    // Not valid Titanfall2 install
                    ElNotification({
                        title: 'Wrong folder',
                        message: "Selected folder is not a valid Titanfall2 install.",
                        type: 'error',
                        position: 'bottom-right'
                    });
                }
            }
        },
        async launchGame(state: any) {
            // TODO update installation if release track was switched
            switch (state.northstar_state) {
                // Install northstar if it wasn't detected.
                case NorthstarState.INSTALL:
                    let install_northstar_result = invoke("install_northstar_caller", { gamePath: state.game_path, northstarPackageName: state.northstar_release_canal });
                    state.northstar_state = NorthstarState.INSTALLING;

                    await install_northstar_result.then((message) => {
                        console.log(message);
                    })
                        .catch((error) => {
                            console.error(error);
                            alert(error);
                        });

                    _get_northstar_version_number(state);
                    break;

                // Update northstar if it is outdated.
                case NorthstarState.MUST_UPDATE:
                    // Updating is the same as installing, simply overwrites the existing files
                    let reinstall_northstar_result = invoke("install_northstar_caller", { gamePath: state.game_path, northstarPackageName: state.northstar_release_canal });
                    state.northstar_state = NorthstarState.UPDATING;

                    await reinstall_northstar_result.then((message) => {
                        console.log(message);
                    })
                        .catch((error) => {
                            console.error(error);
                            alert(error);
                        });

                    _get_northstar_version_number(state);
                    break;

                // Game is ready to play.
                case NorthstarState.READY_TO_PLAY:
                    // Show an error message if Origin is not running.
                    if (!state.origin_is_running) {
                        ElNotification({
                            title: 'Origin is not running',
                            message: "Northstar cannot launch while you're not authenticated with Origin.",
                            type: 'warning',
                            position: 'bottom-right'
                        });

                        // If Origin isn't running, end here
                        return;
                    }

                    let game_install = {
                        game_path: state.game_path,
                        install_type: state.install_type
                    } as GameInstall;
                    await invoke("launch_northstar_caller", { gameInstall: game_install })
                        .then((message) => {
                            console.log(message);
                            // NorthstarState.RUNNING
                        })
                        .catch((error) => {
                            console.error(error);
                            alert(error);
                        });
                    break;

                case NorthstarState.GAME_NOT_FOUND:
                    store.commit('updateGamePath');
                    break;
            }
        }
    }
});

/**
 * This is called when application root component has been mounted.
 * It invokes all Rust methods that are needed to initialize UI.
 */
async function _initializeApp(state: any) {
    // Enable dev mode directly if application is in debug mode
    if (await invoke("is_debug_mode")) {
        state.developer_mode = true;
    }

    // Grab Northstar release canal value from store if exists
    var persistent_northstar_release_canal = (await persistentStore.get('northstar-release-canal')) as any;
    if(persistent_northstar_release_canal) { // For some reason, the plugin-store doesn't throw an eror but simply returns `null` when key not found
        // Put value from peristent store into current store
        state.northstar_release_canal = persistent_northstar_release_canal.value as string;
    }
    else {
        console.log("Value not found in store");
    }

    // Get FlightCore version number
    state.flightcore_version = await invoke("get_version_number");

    const result = await invoke("find_game_install_location_caller")
        .catch((err) => {
            // Gamepath not found or other error
            console.error(err);
            notification_handle = ElNotification({
                title: 'Titanfall2 not found!',
                message: "Please manually select install location",
                type: 'error',
                position: 'bottom-right',
                duration: 0 // Duration `0` means the notification will not auto-vanish
            });
        });
    const typedResult: GameInstall = result as GameInstall;
    state.game_path = typedResult.game_path;
    state.install_type = typedResult.install_type;

    // Check installed Northstar version if found
    await _get_northstar_version_number(state);
}

async function _checkForFlightCoreUpdates(state: FlightCoreStore) {
    // Check if FlightCore up-to-date
    let flightcore_is_outdated = await invoke("check_is_flightcore_outdated_caller") as boolean;

    if (flightcore_is_outdated) {
        ElNotification({
            title: 'FlightCore outdated!',
            message: `Please update FlightCore. Running outdated version ${state.flightcore_version}`,
            type: 'warning',
            position: 'bottom-right',
            duration: 0 // Duration `0` means the notification will not auto-vanish
        });
    }
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
async function _get_northstar_version_number(state: any) {
    let northstar_version_number: string = await invoke("get_northstar_version_number_caller", { gamePath: state.game_path });
    if (northstar_version_number && northstar_version_number.length > 0) {
        state.installed_northstar_version = northstar_version_number;
        state.northstar_state = NorthstarState.READY_TO_PLAY;

        await invoke("check_is_northstar_outdated", { gamePath: state.game_path, northstarPackageName: state.northstar_release_canal })
            .then((message) => {
                if (message) {
                    state.northstar_state = NorthstarState.MUST_UPDATE;
                }
            })
            .catch((error) => {
                console.error(error);
                alert(error);
            });
    }
    else {
        state.northstar_state = NorthstarState.INSTALL;
    }
}
