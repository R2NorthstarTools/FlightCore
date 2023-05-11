/**
 * Implements a fuzzy filter
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
