export default {
    menu: {
        play: 'Play',
        changelog: 'Changelog',
        mods: 'Mods',
        settings: 'Settings',
        dev: 'Dev'
    },

    generic: {
        error: 'Error',
        cancel: 'Cancel',
    },

    play: {
        button: {
            northstar_is_running: 'Game is running',
            select_game_dir: 'Select Titanfall2 game folder',
            install: 'Install',
            installing: 'Installing...',
            update: 'Update',
            updating: 'Updating...',
            ready_to_play: 'Launch game'
        },

        unknown_version: "Unknown version",
        see_patch_notes: "see patch notes",
        players: "players",
        servers: "servers",
        unable_to_load_playercount: "Unable to load playercount",
        northstar_running: "Northstar is running:",
        origin_running: "Origin is running:"
    },

    mods: {
        local: {
            no_mods: "No mods were found.",
            delete_confirm: "Are you sure to delete this mod?",
            delete: "Delete",
            part_of_ts_mod: "This Northstar mod is part of a Thunderstore mod",
            success_deleting: "Success deleting {modName}"
        },

        online: {
            no_match: "No matching mod has been found.",
            try_another_search: "Try another search!"
        },

        menu: {
            local: 'Local',
            online: 'Online',
            filter: 'Filter',
            search: 'Search',
            sort_mods: 'Sort mods',
            select_categories: 'Select categories',

            sort: {
                name_asc: 'By name (A to Z)',
                name_desc: 'By name (Z to A)',
                date_asc: 'By date (from oldest)',
                date_desc: 'By date (from newest)',
                most_downloaded: "Most downloaded",
                top_rated: "Top rated"
            }
        },

        card: {
            button: {
                being_installed: "Installing...",
                being_updated: "Updating...",
                installed: "Installed",
                install: "Install",
                outdated: "Update"
            },

            more_info: "More info",
            remove: "Remove mod",
            remove_dialog_title: "Warning",
            remove_dialog_text: "Delete Thunderstore mod?",
            remove_success: "Removed {modName}",
            install_success: "Installed {modName}"
        }
    },

    settings: {
        manage_install: "Manage installation",
        choose_folder: "Choose installation folder",
        nb_ts_mods_per_page: "Number of Thunderstore mods per page",
        nb_ts_mods_per_page_desc1: "This has an impact on display performances when browsing Thunderstore mods.",
        nb_ts_mods_per_page_desc2: "Set this value to 0 to disable pagination.",
        nb_ts_mods_reset: "Reset to default",
        language: 'Language',
        language_select: "Select your favorite language",
        about: "About:",
        flightcore_version: "FlightCore version:",
        testing: "Testing:",
        enable_test_channels: "Enable testing release channels",
        dev_mode_enabled_title: "Watch out!",
        dev_mod_enabled_text: "Developer mode enabled."
    }
};
