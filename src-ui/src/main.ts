import { invoke } from "@tauri-apps/api";
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';

const $ = document.querySelector.bind(document);

// Stores the overall state of the application
var globalState = {
    gamepath: "",
    installed_northstar_version: "",
    current_view: "" // Note sure if this is the right way to do it
}

document.addEventListener("DOMContentLoaded", async function () {
    // get the elements
    const helloEl = $("div.hello")! as HTMLElement;
    let counterButtonEl = $("counter-button") as HTMLElement;
    let counterResultEl = $("counter-result") as HTMLElement;
    let pingEl = $("backend-ping")! as HTMLElement;
    let panicButtonEl = $("panic-button") as HTMLElement;
    let installLocationHolderEl = $("install-location-holder") as HTMLElement;
    let versionNumberHolderEl = $("version-number-holder") as HTMLElement;
    let omniButtonEl = document.getElementById("omni-button") as HTMLElement;

    // listen backend-ping event (from Tauri Rust App)
    listen("backend-ping", function (evt: TauriEvent<any>) {
        pingEl.classList.add("on");
        setTimeout(function () {
            pingEl.classList.remove("on");
        }, 500);
    })

    // omni button click
    omniButtonEl.addEventListener("click", async function () {
        // Check if Titanfall2 install path as found
        let install_location = await invoke("find_game_install_location_caller") as string;
        if (!(install_location && install_location.length > 0)) {
            alert("Titanfall2 install not found");
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
                alert(selected);


                // TODO Verify if valid Titanfall2 install location
                let is_valid_titanfall2_install = await invoke("verify_install_location", { game_path: selected }) as boolean;
                if (is_valid_titanfall2_install) {
                    globalState.gamepath = selected;

                    // Update omni-button
                    omniButtonEl.textContent = "Install";

                    // Check for Northstar install
                    let northstar_version_number = await invoke("get_northstar_version_number_caller") as string;
                    if (northstar_version_number && northstar_version_number.length > 0) {
                        globalState.installed_northstar_version = northstar_version_number;
                        omniButtonEl.textContent = `Play (${northstar_version_number})`;
                        // Check for updated Northstar
                        let northstar_is_outdated = await invoke("check_is_northstar_outdated") as boolean;
                        if (northstar_is_outdated) {
                            omniButtonEl.textContent = "Update";
                        }
                    }
                }
                else {
                    // Not valid Titanfall2 install
                    alert("Not a valid Titanfall2 install");
                }
            }
            return;
        }

        alert("TODO");
    });

    // counter button click
    counterButtonEl.addEventListener("pointerup", async function () {
        const result = await invoke("add_count", { num: 1 }) as string;
        counterResultEl.textContent = result;
    });

    // hello click
    helloEl.addEventListener("pointerup", async function () {
        const result = await invoke("hello_world") as string;
        helloEl.textContent = result;
        setTimeout(function () {
            helloEl.textContent = "Click again";
        }, 1000);
    })

    // panic button click
    panicButtonEl.addEventListener("pointerup", async function () {
        await invoke("force_panic");
        alert("Never should have been able to get here!");
    });

    // Run the following on initial page load
    // Get version number
    let version_number_string = await invoke("get_version_number") as string;
    versionNumberHolderEl.textContent = version_number_string;

    // Get install location
    let install_location = await invoke("find_game_install_location_caller") as string;
    // Change omni-button content based on whether game install was found
    if (install_location && install_location.length > 0) {
        omniButtonEl.textContent = "Install";
        installLocationHolderEl.textContent = install_location;
        globalState.gamepath = install_location;

        // Check installed Northstar version if found
        let northstar_version_number = await invoke("get_northstar_version_number_caller") as string;
        if (northstar_version_number && northstar_version_number.length > 0) {
            globalState.installed_northstar_version = northstar_version_number;
            omniButtonEl.textContent = `Play (${northstar_version_number})`;
            let northstar_is_outdated = await invoke("check_is_northstar_outdated") as boolean;
            if (northstar_is_outdated) {
                omniButtonEl.textContent = "Update";
            }
        }
        console.log(globalState);
    }
    else {
        omniButtonEl.textContent = "Find Titanfall2 install location";
    }
})
