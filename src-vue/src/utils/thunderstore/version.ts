import {ThunderstoreMod} from "../../../../src-tauri/bindings/ThunderstoreMod";
import {NorthstarMod} from "../../../../src-tauri/bindings/NorthstarMod";
import {store} from "../../plugins/store";

/**
 * Strips off a Thunderstore dependency string from its version
 * (e.g. "taskinoz-WallrunningTitans-1.0.0" to
 * "taskinoz-WallrunningTitans").
 **/
function getThunderstoreDependencyStringPrefix(dependency: string): string {
    const dependencyStringMembers = dependency.split('-');
    return `${dependencyStringMembers[0]}-${dependencyStringMembers[1]}`;
}

function isThunderstoreModOutdated(mod: ThunderstoreMod): boolean {
    // Ensure mod is up-to-date.
    const tsModPrefix = getThunderstoreDependencyStringPrefix(mod.versions[0].full_name);
    const matchingMods: NorthstarMod[] = store.state.installed_mods.filter((mod: NorthstarMod) => {
        if (!mod.thunderstore_mod_string) return false;
        return getThunderstoreDependencyStringPrefix(mod.thunderstore_mod_string!) === tsModPrefix;
    });
    if (matchingMods.length !== 0) {
        // There shouldn't be several mods with same dependency string, but we never know...
        const matchingMod = matchingMods[0];
        // A mod is outdated if its dependency strings differs from Thunderstore dependency string
        // (no need for semver check here)
        return matchingMod.thunderstore_mod_string !== mod.versions[0].full_name;
    }
    return false;
}

export { isThunderstoreModOutdated };
