interface SearchStoreState {
    searchValue: string
}

export const searchModule = {
    state: () => ({
        // This is the treated value of search input
        searchValue: '',
        // Selected mod categories
        selectedCategories: [],
        showDeprecatedMods: false,
        sortValue: {label: '', value: ''}
    }),
    getters: {
        searchWords(state: SearchStoreState): string {
            return state.searchValue.toLowerCase();
        }
    }
  }
