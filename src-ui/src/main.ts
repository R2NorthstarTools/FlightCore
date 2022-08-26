import { invoke } from "@tauri-apps/api";
import { listen, Event as TauriEvent } from "@tauri-apps/api/event";

const $ = document.querySelector.bind(document);

document.addEventListener("DOMContentLoaded", async function () {
    // get the elements
    const helloEl = $("div.hello")! as HTMLElement;
    let counterButtonEl = $("counter-button") as HTMLElement;
    let counterResultEl = $("counter-result") as HTMLElement;
    let pingEl = $("backend-ping")! as HTMLElement;
    let panicButtonEl = $("panic-button") as HTMLElement;

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
})
