import { ComponentCustomProperties } from 'vue'
import { Store } from 'vuex'

declare module '@vue/runtime-core' {
    interface ComponentCustomProperties {
        $i18n: I18n;
        $route: Route;
        $store: Store<FlightCoreStore>;
        $t: (key: string, ...params: any[]) => string;
    }
}
