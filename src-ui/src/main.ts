import { invoke } from "@tauri-apps/api";
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";

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
    let install_location = await invoke("find_game_install_location") as string;
    // Change omni-button content based on whether game install was found
    if (install_location && install_location.length > 0) {
        omniButtonEl.textContent = "Install";
        installLocationHolderEl.textContent = install_location;
        globalState.gamepath = install_location;
    }
    else {
        omniButtonEl.textContent = "Find Titanfall2 install location";
    }
})
