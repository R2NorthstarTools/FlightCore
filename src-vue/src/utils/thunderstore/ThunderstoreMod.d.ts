import { ThunderstoreModVersion } from "./ThunderstoreModVersion";

export interface ThunderstoreMod {
    name: string;
    owner: string;
    date_updated: string;
    rating_score: number;
    package_url: string;
    is_deprecated: boolean;
    versions: ThunderstoreModVersion[];
    categories: string[];
}
