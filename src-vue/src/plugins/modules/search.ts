interface SearchStoreState {
    searchValue: string
}

export const searchModule = {
    state: () => ({
        // This is the treated value of search input
        searchValue: '',
        // Selected mod categories
        selectedCategories: []
    }),
    getters: {
        searchWords(state: SearchStoreState): string {
            return state.searchValue.toLowerCase();
        }
    }
  }