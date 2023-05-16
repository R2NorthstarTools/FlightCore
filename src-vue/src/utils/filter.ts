/**
 * Implements a fuzzy filter
 * Iterates through chars of `search_term` and checks if each char exists in consecutive order in `text`.
 * For example, this means that `text="Gecko"` and `search_term="geo"` will return `true`
 * but using `text="Gecko"` and `search_term="goe"` will return `false`
 * 
 * Implements a subset of "fuzzy string searching"
 * https://en.wikipedia.org/wiki/Approximate_string_matching
 */
function fuzzy_filter(text: string, search_term: string): boolean {
    const lowercase_text = text.toLowerCase();
    const lowercase_search_term = search_term.toLowerCase();

    let previousIndex = -1;
    for (let i = 0; i < lowercase_search_term.length; i++) {
        const char = lowercase_search_term[i];
        const currentIndex = lowercase_text.indexOf(char, previousIndex + 1);
        if (currentIndex === -1) {
            return false;
        }
        previousIndex = currentIndex;
    }

    return true;
}
export { fuzzy_filter };
