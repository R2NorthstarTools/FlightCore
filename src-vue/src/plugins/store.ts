import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { Tabs } from "../utils/Tabs";
import { InstallType } from "../utils/InstallType";
import { invoke } from "@tauri-apps/api";
import { GameInstall } from "../utils/GameInstall";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { FlightCoreVersion } from "../utils/FlightCoreVersion";
import { ElNotification, NotificationHandle } from 'element-plus';
import { NorthstarState } from '../utils/NorthstarState';
import { appDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';
import { Store } from 'tauri-plugin-store-api';
import { router } from "../main";
import ReleaseInfo from "../utils/ReleaseInfo";
import { ThunderstoreMod } from '../utils/thunderstore/ThunderstoreMod';
import { NorthstarMod } from "../utils/NorthstarMod";

const persistentStore = new Store('flight-core-settings.json');


export interface FlightCoreStore {
    developer_mode: boolean,
    game_path: string,
    install_type: InstallType,

    flightcore_version: string,

    installed_northstar_version: string,
    northstar_state: NorthstarState,
    northstar_release_canal: ReleaseCanal,
    enableReleasesSwitch: boolean,
    releaseNotes: ReleaseInfo[],

    thunderstoreMods: ThunderstoreMod[],
    installed_mods: NorthstarMod[],

    northstar_is_running: boolean,
    origin_is_running: boolean,

    // user custom settings
    mods_per_page: number,
}

let notification_handle: NotificationHandle;

export const store = createStore<FlightCoreStore>({
    state(): FlightCoreStore {
        return {
            developer_mode: false,
            game_path: undefined as unknown as string,
            install_type: undefined as unknown as InstallType,

            flightcore_version: "",

            installed_northstar_version: "",
            northstar_state: NorthstarState.GAME_NOT_FOUND,
            northstar_release_canal: ReleaseCanal.RELEASE,
            enableReleasesSwitch: false,
            releaseNotes: [],

            thunderstoreMods: [],
            installed_mods: [],

            northstar_is_running: false,
            origin_is_running: false,

            mods_per_page: 20,
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
            router.push({ path: newTab });
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
                    try {
                        notification_handle.close();
                    }
                    catch {
                        console.warn("Nothing to close");
                    }
                    state.install_type = InstallType.UNKNOWN;

                    let game_install = {
                        game_path: selected,
                        install_type: InstallType.UNKNOWN
                    } as GameInstall;

                    // Save change in persistent store
                    await persistentStore.set('game-install', { value: game_install });

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
        async launchGame(state: any, no_checks = false) {
            let game_install = {
                game_path: state.game_path,
                install_type: state.install_type
            } as GameInstall;

            if (no_checks) {
                await invoke("launch_northstar_caller", { gameInstall: game_install, bypassChecks: no_checks })
                    .then((message) => {
                        console.log("Launched with bypassed checks");
                        console.log(message);
                    })
                    .catch((error) => {
                        console.error(error);
                        alert(error);
                    });

                return;
            }

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
                    await invoke("launch_northstar_caller", { gameInstall: game_install })
                        .then((message) => {
                            console.log(message);
                            // NorthstarState.RUNNING
                        })
                        .catch((error) => {
                            console.error(error);
                            ElNotification({
                                title: 'Error',
                                message: error,
                                type: 'error',
                                position: 'bottom-right'
                            });
                        });
                    break;

                case NorthstarState.GAME_NOT_FOUND:
                    store.commit('updateGamePath');
                    break;
            }
        },
        async fetchReleaseNotes(state: FlightCoreStore) {
            if (state.releaseNotes.length !== 0) return;
            state.releaseNotes = await invoke("get_northstar_release_notes");
        },
        async fetchThunderstoreMods(state: FlightCoreStore) {
            // To check if some Thunderstore mods are already installed/outdated, we need to load locally-installed mods.
            await store.commit('loadInstalledMods');
            if (state.thunderstoreMods.length !== 0) return;

            const response = await fetch('https://northstar.thunderstore.io/api/v1/package/');
            let mods = JSON.parse(await (await response.blob()).text());

            // Remove some mods from listing
            const removedMods = ['Northstar', 'NorthstarReleaseCandidate', 'r2modman'];
            state.thunderstoreMods = mods.filter((mod: ThunderstoreMod) => {
                return !removedMods.includes(mod.name) && !mod.is_deprecated;
            });
        },
        async loadInstalledMods(state: FlightCoreStore) {
            let game_install = {
                game_path: state.game_path,
                install_type: state.install_type
            } as GameInstall;

            // If there's no game path, prevent looking for installed mods.
            if (state.game_path === undefined) {
                console.warn('Cannot load installed mods since so game path is selected.');
                return;
            }

            // Call back-end for installed mods
            await invoke("get_installed_mods_and_properties", { gameInstall: game_install })
                .then((message) => {
                    state.installed_mods = (message as NorthstarMod[]);
                })
                .catch((error) => {
                    console.error(error);
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                });
        },
        async toggleReleaseCandidate(state: FlightCoreStore) {
            // Flip between RELEASE and RELEASE_CANDIDATE
            state.northstar_release_canal = state.northstar_release_canal === ReleaseCanal.RELEASE
                ? ReleaseCanal.RELEASE_CANDIDATE
                : ReleaseCanal.RELEASE;

            // Save change in persistent store
            await persistentStore.set('northstar-release-canal', { value: state.northstar_release_canal });

            // Update current state so that update check etc can be performed
            store.commit("checkNorthstarUpdates");

            // Display notification to highlight change
            ElNotification({
                title: `${state.northstar_release_canal}`,
                message: `Switched release channel to: "${state.northstar_release_canal}"`,
                type: 'success',
                position: 'bottom-right'
            });
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

        // Make menubar striped if debug build
        let menu_bar_handle = document.querySelector('#fc_menu-bar');
        console.log(menu_bar_handle);
        if (menu_bar_handle !== null) {
            menu_bar_handle.classList.toggle('developer_build');
        }
    } else {
        // Disable context menu in release build.
        document.addEventListener('contextmenu', event => event.preventDefault());
    }

    // Grab Northstar release canal value from store if exists
    var persistent_northstar_release_canal = (await persistentStore.get('northstar-release-canal')) as any;
    if (persistent_northstar_release_canal) { // For some reason, the plugin-store doesn't throw an eror but simply returns `null` when key not found
        // Put value from peristent store into current store
        state.northstar_release_canal = persistent_northstar_release_canal.value as string;
    }
    else {
        console.log("Value not found in store");
    }

    // Grab "Enable releases switching" setting from store if possible
    const valueFromStore: { value: boolean } | null = await persistentStore.get('northstar-releases-switching');
    if (valueFromStore) {
        state.enableReleasesSwitch = valueFromStore.value;
    }

    // Grab "Thunderstore mods per page" setting from store if possible
    const perPageFromStore: { value: number } | null = await persistentStore.get('thunderstore-mods-per-page');
    if (perPageFromStore && perPageFromStore.value) {
        state.mods_per_page = perPageFromStore.value;
    }

    // Get FlightCore version number
    state.flightcore_version = await invoke("get_flightcore_version_number");

    var result = undefined;
    var persistent_game_install = (await persistentStore.get('game-install')) as any;

    if ( // Safety checks for value from store
        persistent_game_install
        && persistent_game_install.value !== undefined
        && persistent_game_install.value.game_path !== undefined
        && persistent_game_install.value.install_type !== undefined
    ) { // For some reason, the plugin-store doesn't throw an eror but simply returns `null` when key not found
        let game_install = persistent_game_install.value as GameInstall;
        // check if valid path
        let is_valid_titanfall2_install = await invoke("verify_install_location", { gamePath: game_install.game_path }) as boolean;
        if (is_valid_titanfall2_install) {
            // Use value from peristent store
            result = game_install;
        }

    }

    if (result === undefined) { // No (valid) value found in persistent store
        result = await invoke("find_game_install_location_caller")
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
    }

    if (result !== undefined) { // Found some form of value for gameinstall

        const typedResult: GameInstall = result as GameInstall;

        // Save change in persistent store
        await persistentStore.set('game-install', { value: typedResult });

        // Update UI store
        state.game_path = typedResult.game_path;
        state.install_type = typedResult.install_type;

        // Check installed Northstar version if found
        await _get_northstar_version_number(state);
    }
}

async function _checkForFlightCoreUpdates(state: FlightCoreStore) {
    // Check if FlightCore up-to-date
    let flightcore_is_outdated = await invoke("check_is_flightcore_outdated_caller") as boolean;

    if (flightcore_is_outdated) {
        let newest_flightcore_version = await invoke("get_newest_flightcore_version") as FlightCoreVersion;
        ElNotification({
            title: 'FlightCore outdated!',
            message: `Please update FlightCore.\nRunning outdated version ${state.flightcore_version}.\nNewest is ${newest_flightcore_version.tag_name}!`,
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
    await invoke("get_northstar_version_number_caller", { gamePath: state.game_path })
        .then((message) => {
            let northstar_version_number: string = message as string;
            state.installed_northstar_version = northstar_version_number;
            state.northstar_state = NorthstarState.READY_TO_PLAY;

            invoke("check_is_northstar_outdated", { gamePath: state.game_path, northstarPackageName: state.northstar_release_canal })
                .then((message) => {
                    if (message) {
                        state.northstar_state = NorthstarState.MUST_UPDATE;
                    }
                })
                .catch((error) => {
                    console.error(error);
                    alert(error);
                });
        })
        .catch((error) => {
            state.northstar_state = NorthstarState.INSTALL;
        })
}
