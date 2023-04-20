import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { Tabs } from "../utils/Tabs";
import { InstallType } from "../utils/InstallType";
import { invoke } from "@tauri-apps/api";
import { GameInstall } from "../utils/GameInstall";
import { ReleaseCanal } from "../utils/ReleaseCanal";
import { FlightCoreVersion } from "../../../src-tauri/bindings/FlightCoreVersion";
import { NotificationHandle } from 'element-plus';
import { NorthstarState } from '../utils/NorthstarState';
import { appDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';
import { Store } from 'tauri-plugin-store-api';
import { router } from "../main";
import { ReleaseInfo } from "../../../src-tauri/bindings/ReleaseInfo";
import { ThunderstoreMod } from "../../../src-tauri/bindings/ThunderstoreMod";
import { NorthstarMod } from "../../../src-tauri/bindings/NorthstarMod";
import { searchModule } from './modules/search';
import { i18n } from '../main';
import { pullRequestModule } from './modules/pull_requests';
import { showErrorNotification, showNotification } from '../utils/ui';
import { ElMessageBox } from "element-plus";

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
    thunderstoreModsCategories: string[],
    installed_mods: NorthstarMod[],

    northstar_is_running: boolean,
    origin_is_running: boolean,

    player_count: number,
    server_count: number,

    // user custom settings
    mods_per_page: number,

    can_install_plugins: boolean,
}

let notification_handle: NotificationHandle;


export const store = createStore<FlightCoreStore>({
    modules: {
        search: searchModule,
        pullrequests: pullRequestModule,
    },
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
            thunderstoreModsCategories: [],
            installed_mods: [],

            northstar_is_running: false,
            origin_is_running: false,

            player_count: -1,
            server_count: -1,

            mods_per_page: 20,

            can_install_plugins: false,
        }
    },
    mutations: {
        checkNorthstarUpdates(state) {
            _get_northstar_version_number(state);
        },
        async toggleDebugMode(_state) {
            let menu_bar_handle = document.querySelector('#fc_menu-bar');
            if (menu_bar_handle !== null) {
                menu_bar_handle.classList.toggle('developer_build');
            }
        },
        async toggleDeveloperMode(state) {
            state.developer_mode = !state.developer_mode;

            // Reset tab when closing dev mode.
            if (!state.developer_mode) {
                store.commit('updateCurrentTab', Tabs.PLAY);
            }

            // Save dev mode state in persistent store
            await persistentStore.set('dev_mode', state.developer_mode);
            await persistentStore.save();
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
                    showNotification(
                        i18n.global.tc('notification.game_folder.new.title'),
                        i18n.global.tc('notification.game_folder.new.text')
                    );
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
                    await persistentStore.save(); // explicit save to disk

                    // Check for Northstar install
                    store.commit('checkNorthstarUpdates');
                }
                else {
                    // Not valid Titanfall2 install
                    showErrorNotification(
                        i18n.global.tc('notification.game_folder.wrong.text'),
                        i18n.global.tc('notification.game_folder.wrong.title')
                    );
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
                            showErrorNotification(error);
                        });
                    break;

                case NorthstarState.GAME_NOT_FOUND:
                    store.commit('updateGamePath');
                    break;
            }
        },
        async launchGameSteam(state: any, no_checks = false) {
            let game_install = {
                game_path: state.game_path,
                install_type: state.install_type
            } as GameInstall;

            await invoke("launch_northstar_steam_caller", { gameInstall: game_install, bypassChecks: no_checks })
                .then((message) => {
                    showNotification('Success');
                })
                .catch((error) => {
                    console.error(error);
                    showErrorNotification(error);
                });

            return;
        },
        async fetchReleaseNotes(state: FlightCoreStore) {
            if (state.releaseNotes.length !== 0) return;
            state.releaseNotes = await invoke("get_northstar_release_notes");
        },
        async fetchThunderstoreMods(state: FlightCoreStore) {
            // To check if some Thunderstore mods are already installed/outdated, we need to load locally-installed mods.
            await store.commit('loadInstalledMods');
            if (state.thunderstoreMods.length !== 0) return;

            let mods: ThunderstoreMod[] = [];
            await invoke<ThunderstoreMod[]>("query_thunderstore_packages_api")
                .then((message) => {
                    mods = message;
                })
                .catch((error) => {
                    console.error(error);
                    return;
                });

            if (mods == undefined) {
                return;
            }

            // Remove some mods from listing
            state.thunderstoreMods = mods;

            // Retrieve categories from mods
            state.thunderstoreModsCategories = mods
                .map((mod: ThunderstoreMod) => mod.categories)
                .filter((modCategories: string[]) => modCategories.length !== 0)
                .reduce((accumulator: string[], modCategories: string[]) => {
                    accumulator.push( ...modCategories.filter((cat: string) => !accumulator.includes(cat)) );
                    return accumulator;
                }, [])
                .sort();
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
                    showErrorNotification(error);
                });
        },
        async toggleReleaseCandidate(state: FlightCoreStore) {
            // Flip between RELEASE and RELEASE_CANDIDATE
            state.northstar_release_canal = state.northstar_release_canal === ReleaseCanal.RELEASE
                ? ReleaseCanal.RELEASE_CANDIDATE
                : ReleaseCanal.RELEASE;

            // Save change in persistent store
            await persistentStore.set('northstar-release-canal', { value: state.northstar_release_canal });
            await persistentStore.save(); // explicit save to disk

            // Update current state so that update check etc can be performed
            store.commit("checkNorthstarUpdates");

            // Display notification to highlight change
            showNotification(
                i18n.global.tc(`channels.names.${state.northstar_release_canal}`),
                i18n.global.tc('channels.release.switch.text', {canal: state.northstar_release_canal}),
            );
        }
    }
});

/**
 * This is called when application root component has been mounted.
 * It invokes all Rust methods that are needed to initialize UI.
 */
async function _initializeApp(state: any) {
    // Display dev view if dev mode was previously enabled.
    const devModeEnabled: boolean = await persistentStore.get('dev_mode') ?? false;
    const debugModeEnabled: boolean = await invoke("is_debug_mode");
    if (devModeEnabled) {
        store.commit('toggleDeveloperMode');
    }
    if (debugModeEnabled) {
        store.commit('toggleDebugMode');
    }

    // Disable context menu in release build.
    if (!debugModeEnabled) {
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
                notification_handle = showNotification(
                    i18n.global.tc('notification.game_folder.not_found.title'),
                    i18n.global.tc('notification.game_folder.not_found.text'),
                    'error',
                    0   // Duration `0` means the notification will not auto-vanish
                );
            });
    }

    if (result !== undefined) { // Found some form of value for gameinstall

        const typedResult: GameInstall = result as GameInstall;

        // Save change in persistent store
        await persistentStore.set('game-install', { value: typedResult });
        await persistentStore.save(); // explicit save to disk

        // Update UI store
        state.game_path = typedResult.game_path;
        state.install_type = typedResult.install_type;

        // Check installed Northstar version if found
        await _get_northstar_version_number(state);
    }

    await invoke<[number, number]>("get_server_player_count")
        .then((message) => {
            state.player_count = message[0];
            state.server_count = message[1];
        })
        .catch((error) => {
            console.warn("Failed getting player/server count");
            console.warn(error);
        });
}

async function _checkForFlightCoreUpdates(state: FlightCoreStore) {
    // Check if FlightCore up-to-date
    let flightcore_is_outdated = await invoke("check_is_flightcore_outdated_caller") as boolean;

    if (flightcore_is_outdated) {
        let newest_flightcore_version = await invoke("get_newest_flightcore_version") as FlightCoreVersion;
        showNotification(
            i18n.global.tc('notification.flightcore_outdated.title'),
            i18n.global.tc('notification.flightcore_outdated.text', {oldVersion: state.flightcore_version, newVersion: newest_flightcore_version.tag_name}),
            'warning',
            0 // Duration `0` means the notification will not auto-vanish
        );
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

    listen("northstar-statistics", function (evt: TauriEvent<{Ok: [number, number]}>) {
        state.player_count = evt.payload.Ok[0];
        state.server_count = evt.payload.Ok[1];
    });

    listen("display-plugin-warning", async function (evt: TauriEvent<any>) {
        await display_plugin_warning() // could also disaplay the names of the plugins
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

async function display_plugin_warning() {
    ElMessageBox.alert("This mod contains a plugin. Plugins CAN BE REALLY DANGEROURS since they have to access to your system!", "Warning");

    // wait 5 seconds
    await new Promise((resolve) => setTimeout(resolve, 5000));

    await ElMessageBox.confirm("Do you still want to install this mod with a plugin?", "Warning", { confirmButtonText: "YES", cancelButtonText: "Cancel", type: "warning", })
        .then(() => {
            invoke("receive_install_status", { comfirmedInstall: true })
        })
        .catch(() => {
            invoke("receive_install_status", { comfirmedInstall: false })
        })
}