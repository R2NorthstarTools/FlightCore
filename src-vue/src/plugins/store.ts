import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { Tabs } from "../utils/Tabs";
import { InstallType } from "../utils/InstallType";
import { invoke } from "@tauri-apps/api";
import { GameInstall } from "../utils/GameInstall";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { ElNotification } from 'element-plus';
import { NorthstarState } from '../utils/NorthstarState';


export interface FlightCoreStore {
    current_tab: Tabs,
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

export const store = createStore<FlightCoreStore>({
    state (): FlightCoreStore {
        return {
            current_tab: Tabs.PLAY,
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
            state.current_tab = newTab;
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

    // Get FlightCore version number
    state.flightcore_version = await invoke("get_version_number");

    const result = await invoke("find_game_install_location_caller")
        .catch((err) => {
            // Gamepath not found or other error
            console.error(err);
            ElNotification({
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
