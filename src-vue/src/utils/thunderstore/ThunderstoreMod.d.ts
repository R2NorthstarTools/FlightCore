import { ThunderstoreModVersion } from "./ThunderstoreModVersion";

export interface ThunderstoreMod {
    name: string;
    owner: string;
    versions: ThunderstoreModVersion[];
}
