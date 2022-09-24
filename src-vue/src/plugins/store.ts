import { createStore } from 'vuex';
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";

export const store = createStore({
    state () {
      return {
        developer_mode: false,
        
        installed_northstar_version: "1.9.7",

        northstar_is_running: false,
        origin_is_running: false
      }
    },
    mutations: {
        toggleDeveloperMode(state) {
            state.developer_mode = !state.developer_mode;
        },
        initializeListeners(state) {
            _initializeListeners(state);
        }
    }
});

function _initializeListeners(state: any) {
    listen("origin-running-ping", function (evt: TauriEvent<any>) {
        state.origin_is_running = evt.payload as boolean;
        console.log(`Origin is running: ${evt.payload}`);
    });

    listen("northstar-running-ping", function (evt: TauriEvent<any>) {
        state.northstar_is_running = evt.payload as boolean;
        console.log(`Northstar is running: ${evt.payload}`);
    });
}