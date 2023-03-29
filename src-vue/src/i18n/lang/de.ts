export default {
    menu: {
        play: 'Spielen',
        changelog: 'Versionhistorie',
        mods: 'Mods',
        settings: 'Einstellungen',
        dev: 'Dev'
    },

    generic: {
        yes: 'Ja',
        no: 'Nein',
        error: 'Fehler',
        cancel: 'Abbrechen',
        informationShort: 'Info'
    },

    play: {
        button: {
            northstar_is_running: 'Spiel läuft',
            select_game_dir: 'Titanfall2 ordner wählen',
            install: 'Installieren',
            installing: 'Installiert...',
            update: 'Aktualisieren',
            updating: 'Aktualisiert...',
            ready_to_play: 'Spiel starten'
        },

        unknown_version: "Unbekannte Version",
        see_patch_notes: "Siehe Patch-Notizen",
        players: "Spieler",
        servers: "Server",
        unable_to_load_playercount: "Spielerzahl konnte nicht geladen werden",
        northstar_running: "Northstar läuft:",
        origin_running: "Origin läuft:"
    },

    mods: {
        local: {
            no_mods: "Keine Mods gefunden.",
            delete_confirm: "Bist du dir sicher, dass du diesen Mod löschen möchtest?",
            delete: "Löschen",
            part_of_ts_mod: "Dieser Northstar Mod ist teil eines Thunderstore Mods",
            success_deleting: "{modName} erfolgreich gelöscht"
        },

        online: {
            no_match: "Keine passenden Mods gefunden.",
            try_another_search: "Versuche eine andere Suchanfrage!"
        },

        menu: {
            local: 'Lokal',
            online: 'Online',
            filter: 'Filter',
            search: 'Suche',
            sort_mods: 'Mods sortieren',
            select_categories: 'Kategorien wählen',

            sort: {
                name_asc: 'Nach Name (A to Z)',
                name_desc: 'Nach Name (Z to A)',
                date_asc: 'Nach Datum (älteste zuerst)',
                date_desc: 'Nach Datum (neuste zuerst)',
                most_downloaded: "Am meisten heruntergeladen",
                top_rated: "Am besten bewerted"
            }
        },

        card: {
            button: {
                being_installed: "Installiert...",
                being_updated: "Aktualisiert...",
                installed: "Installiert",
                install: "Installieren",
                outdated: "Aktualisieren"
            },

            by: "von",
            more_info: "Mehr Info",
            remove: "Mod entfernen",
            remove_dialog_title: "Warnung",
            remove_dialog_text: "Thunderstore Mod entfernen?",
            remove_success: "{modName} entfernt",
            install_success: "{modName} installiert"
        }
    },

    settings: {
        manage_install: "Installation verwalten",
        choose_folder: "Installationsordner wählen",
        nb_ts_mods_per_page: "Anzahl an Thunderstore Mods pro Seite",
        nb_ts_mods_per_page_desc1: "Ändern dieser Einstellung kann die Leistung beim Suchen von Thunderstore Mods beeinflussen.",
        nb_ts_mods_per_page_desc2: "Setze diesen Wert auf 0 um Paginierung zu deaktivieren",
        nb_ts_mods_reset: "Standard wiederherstellen",
        language: 'Sprache',
        language_select: "Bevorzugte Sprache wählen",
        about: "Über:",
        flightcore_version: "FlightCore Version:",
        testing: "Testen:",
        enable_test_channels: "Testversionen aktivieren",
        dev_mode_enabled_title: "Vorsicht!",
        dev_mod_enabled_text: "Entwicklermodus aktiviert.",

        repair: {
            title: "Reparieren",
            open_window: "Reparierfenster öffnen",

            window: {
                title: "FlightCore Reparierfenster",
                warning: "Dieses Fenster enthält verschiedene Funktionen um gängige Probleme mit Northstar und FlightCore zu beheben.",
                disable_all_but_core: "Alle außer notwendige Mods deaktivieren",
                force_reinstall_ns: "Northstar reinstallieren",
                force_delete_temp_dl: "Temporären FlightCore Downloadordner löschen",
                delete_persistent_store: "FlightCore Einstellungen zurücksetzen"
            }
        }
    },

    notification: {
        game_folder: {
            new: {
                title: "Neuer Spielordner",
                text: "Spielordner erfolgreich aktualisiert."
            },

            wrong: {
                title: "Falscher Ordner",
                text: "Der gewählte Ordner enthält eine valide Titanfall2 Installation."
            },

            not_found: {
                title: "Titanfall2 nicht gefunden!",
                text: "Bitte wähle den Installationsordner manuell aus"
            }
        },

        flightcore_outdated: {
            title: "FlightCore veraltet!",
            text: "Bitte aktualisiere FlightCore.\nDu hast die veraltetet Version {oldVersion}.\nNeuste Version ist {newVersion}!"
        }
    },

    channels: {
        release: {
            switch: {
                text: 'Releasekanal zu "{canal} gewechselt".'
            }
        },

        names: {
            Northstar: 'Northstar',
            NorthstarReleaseCandidate: 'Northstar Release Candidate'
        }
    }
};