// Enumerates the way Titanfall2 could be installed (Steam/Origin/EA-Desktop)
export enum InstallType {
    STEAM = 'STEAM',
    ORIGIN = 'ORIGIN',
    EAPLAY = 'EAPLAY',
    UNKNOWN = 'UNKNOWN', // used when the install location was manually selected
}
