import createPlugin, { Plugin } from "@extism/extism";
import {createResource} from "solid-js";

const getPlugin = async () : Promise<Plugin> => {
    return await createPlugin({
        wasm: [{
            url: 'http://localhost:8080/plugins.wasm'
        }]
    }, {useWasi : true})
}

export const [plugin, {}] = createResource(getPlugin)
