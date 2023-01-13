<template>
    <el-card :body-style="{ padding: '0px' }">
        <img
            :src="latestVersion.icon"
            class="image"
        />
        <div style="padding: 0 10px 10px;">
            <span class="statContainer">
                <el-icon class="no-inherit">
                    <Download />
                </el-icon>
                {{ modDownloadsCount }}
            </span>

            <span class="statContainer">
                {{ mod.rating_score }}
                <el-icon class="no-inherit">
                    <Star />
                </el-icon>
            </span>
            <br/>

            <div class="name hide-text-overflow">{{ mod.name }}</div>
            <div class="author hide-text-overflow">by {{ mod.owner }}</div>
            <div class="desc">
                {{ latestVersion.description }}
            </div>

            <span style="display: flex">
                <el-button
                    :type="modButtonType"
                    style="flex: 6"
                    :loading="isBeingInstalled || isBeingUpdated"
                    @click.stop="installMod(mod)"
                >
                    {{ modButtonText }}
                </el-button>

                <!-- Information dropdown menu -->
                 <el-button v-if="!modIsRemovable"
                            link type="info" class="infoBtn" @click="openURL(mod.package_url)">
                    <el-icon>
                        <InfoFilled />
                    </el-icon>
                </el-button>

                <el-dropdown v-else>
                    <el-icon class="infoBtn moreBtn">
                        <MoreFilled />
                    </el-icon>
                    <template #dropdown>
                        <el-dropdown-menu>
                            <el-dropdown-item @click="openURL(mod.package_url)">
                                More info
                            </el-dropdown-item>
                            <el-dropdown-item  @click="deleteMod(mod)">
                                Remove mod
                            </el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
            </span>
        </div>
    </el-card>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import {ThunderstoreMod} from "../utils/thunderstore/ThunderstoreMod";
import {ThunderstoreModVersion} from "../utils/thunderstore/ThunderstoreModVersion";
import {invoke, shell} from "@tauri-apps/api";
import {ThunderstoreModStatus} from "../utils/thunderstore/ThunderstoreModStatus";
import {NorthstarMod} from "../utils/NorthstarMod";
import {GameInstall} from "../utils/GameInstall";
import {ElNotification} from "element-plus";
import { NorthstarState } from "../utils/NorthstarState";
import { ElMessageBox } from "element-plus";

export default defineComponent({
    name: "ThunderstoreModCard",
    props: {
        mod: {
            required: true,
            type: Object as () => ThunderstoreMod
        }
    },
    data: () => ({
        isBeingInstalled: false,
        isBeingUpdated: false
    }),
    computed: {
        latestVersion (): ThunderstoreModVersion {
            return this.mod.versions[0];
        },

        /**
         * Returns the status of a given mod.
         */
        modStatus(): ThunderstoreModStatus {
            if (this.isBeingInstalled) {
                return ThunderstoreModStatus.BEING_INSTALLED;
            }
            if (this.isBeingUpdated) {
                return ThunderstoreModStatus.BEING_UPDATED;
            }

            // Ensure mod is up-to-date.
            const tsModPrefix = this.getThunderstoreDependencyStringPrefix(this.latestVersion.full_name);
            const matchingMods: NorthstarMod[] = this.$store.state.installed_mods.filter((mod: NorthstarMod) => {
                if (!mod.thunderstore_mod_string) return false;
                return this.getThunderstoreDependencyStringPrefix(mod.thunderstore_mod_string!) === tsModPrefix;
            });
            if (matchingMods.length !== 0) {
                // There shouldn't be several mods with same dependency string, but we never know...
                const matchingMod = matchingMods[0];
                // A mod is outdated if its dependency strings differs from Thunderstore dependency string
                // (no need for semver check here)
                return matchingMod.thunderstore_mod_string === this.latestVersion.full_name
                    ? ThunderstoreModStatus.INSTALLED
                    : ThunderstoreModStatus.OUTDATED;
            }

            return ThunderstoreModStatus.NOT_INSTALLED;
        },

        /**
         * Returns button text associated to a mod.
         */
        modButtonText(): string {
            switch (this.modStatus) {
                case ThunderstoreModStatus.BEING_INSTALLED:
                    return "Installing...";
                case ThunderstoreModStatus.BEING_UPDATED:
                    return "Updating...";
                case ThunderstoreModStatus.INSTALLED:
                    return "Installed";
                case ThunderstoreModStatus.NOT_INSTALLED:
                    return "Install";
                case ThunderstoreModStatus.OUTDATED:
                    return "Update";
            }
        },

        /**
         * Returns button type associated to a mod.
         */
        modButtonType(): string {
            switch (this.modStatus) {
                case ThunderstoreModStatus.BEING_INSTALLED:
                    return "primary";
                case ThunderstoreModStatus.INSTALLED:
                    return "success";
                case ThunderstoreModStatus.NOT_INSTALLED:
                    return "primary";
                case ThunderstoreModStatus.OUTDATED:
                case ThunderstoreModStatus.BEING_UPDATED:
                    return "warning";
            }
        },

        /**
         * Tells if a Thunderstore mod can be removed.
         * This is used to tell if we should display the "Remove mod" option.
         **/
        modIsRemovable(): boolean {
            return [ThunderstoreModStatus.INSTALLED, ThunderstoreModStatus.OUTDATED]
                .includes(this.modStatus);
        },

        /**
         * This computes the total count of downloads of a given mod, by adding
         * download count of each of its releases.
         */
        modDownloadsCount(): number {
            let totalDownloads = 0;
            this.mod.versions.map((version: ThunderstoreModVersion) => totalDownloads += version.downloads);
            return totalDownloads;
        },
    },
    methods: {
        /**
         * This opens an URL in user's favorite web browser.
         * This is used to open Thunderstore mod pages.
         */
        openURL(url: string): void {
            shell.open(url);
        },

        /**
         * Strips off a Thunderstore dependency string from its version
         * (e.g. "taskinoz-WallrunningTitans-1.0.0" to
         * "taskinoz-WallrunningTitans").
         */
        getThunderstoreDependencyStringPrefix (dependency: string): string {
            const dependencyStringMembers = dependency.split('-');
            return `${dependencyStringMembers[0]}-${dependencyStringMembers[1]}`;
        },

        async deleteMod(mod: ThunderstoreMod) {

            // Show pop-up to confirm delete
            ElMessageBox.confirm(
                'Delete Thunderstore mod?',
                'Warning',
                {
                    confirmButtonText: 'OK',
                    cancelButtonText: 'Cancel',
                    type: 'warning',
                }
            )
                .then(async () => { // Deletion confirmed
                    let game_install = {
                        game_path: this.$store.state.game_path,
                        install_type: this.$store.state.install_type
                    } as GameInstall;

                    await invoke("delete_thunderstore_mod", { gameInstall: game_install, thunderstoreModString: this.latestVersion.full_name })
                        .then((message) => {
                            ElNotification({
                                title: `Removed ${mod.name}`,
                                message: message as string,
                                type: 'success',
                                position: 'bottom-right'
                            });
                        })
                        .catch((error) => {
                            ElNotification({
                                title: 'Error',
                                message: error,
                                type: 'error',
                                position: 'bottom-right'
                            });
                        })
                        .finally(() => {
                            this.$store.commit('loadInstalledMods');
                        });
                })
                .catch(() => { // Deletion cancelled
                    console.log("Deleting Thunderstore mod cancelled.")
                })
        },

        async installMod (mod: ThunderstoreMod) {
            let game_install = {
                game_path: this.$store.state.game_path,
                install_type: this.$store.state.install_type
            } as GameInstall;

            // set internal state according to current installation state
            if (this.modStatus === ThunderstoreModStatus.OUTDATED) {
                this.isBeingUpdated = true;
            } else {
                this.isBeingInstalled = true;
            }

            await invoke("install_mod_caller", { gameInstall: game_install, thunderstoreModString: this.latestVersion.full_name }).then((message) => {
                ElNotification({
                    title: `Installed ${mod.name}`,
                    message: message as string,
                    type: 'success',
                    position: 'bottom-right'
                });
            })
                .catch((error) => {
                    ElNotification({
                        title: 'Error',
                        message: error,
                        type: 'error',
                        position: 'bottom-right'
                    });
                })
                .finally(() => {
                    this.isBeingInstalled = false;
                    this.isBeingUpdated = false;
                    this.$store.commit('loadInstalledMods');
                });
        },
    }
});
</script>

<style scoped>
.el-card {
    display: inline-block;
    max-width: 178px;
    margin: 5px;
}

.author {
    font-size: 14px;
    font-style: italic;
}

.hide-text-overflow {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
}

.desc {
    font-size: 12px;
    margin: 8px 0 16px;
    height: 57px;
    text-overflow: ellipsis;
    overflow: hidden;
}

.statContainer {
    font-size: 14px;
}

.statContainer:nth-child(2) {
    float: right;
}

.infoBtn {
    width: 20px;
    padding: 0 !important;
    font-size: 20px;
    border: none;
}

.moreBtn {
    margin-left: 10px;
    height: auto;
}
</style>
