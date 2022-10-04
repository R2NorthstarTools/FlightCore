// Enumerates the way Titanfall2 could be installed (Steam/Origin/EA-Desktop)
// Needs to be synced with `pub enum InstallType` in /src-tauri/src/lib.rs
export enum InstallType {
    STEAM = 'STEAM',
    ORIGIN = 'ORIGIN',
    EAPLAY = 'EAPLAY',
    UNKNOWN = 'UNKNOWN', // used when the install location was manually selected
}
