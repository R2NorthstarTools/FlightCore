import { ThunderstoreModVersion } from "./ThunderstoreModVersion";

export interface ThunderstoreMod {
    name: string;
    owner: string;
    rating_score: number;
    versions: ThunderstoreModVersion[];
}
