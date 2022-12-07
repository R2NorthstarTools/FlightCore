import { ThunderstoreModVersion } from "./ThunderstoreModVersion";

export interface ThunderstoreMod {
    name: string;
    owner: string;
    rating_score: number;
    package_url: string;
    is_deprecated: boolean;
    versions: ThunderstoreModVersion[];
}
