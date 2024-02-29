import createPlugin, { Plugin } from "@extism/extism";
import {createStore} from "solid-js/store";

const PLUGIN = 'plugin'
interface State {
    [PLUGIN]: Plugin | null
}

export const [store, setStore] = createStore<State>({
    plugin : null
})
