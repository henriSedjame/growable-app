import createPlugin, {CallContext, Plugin} from "@extism/extism";
import {createResource} from "solid-js";
import {createStore} from "solid-js/store";

const EXTISM_USER = 'extism:host/user'
enum PluginStoreAttrs {
    current = 'current',
    plugins = 'plugins'
}

interface PluginStore {
    [PluginStoreAttrs.current]: string | null,
    [PluginStoreAttrs.plugins]: Map<string, Plugin>
}


export const [plugin_store, setPluginStore] = createStore<PluginStore>( {
    current: null,
    plugins: new Map()
})

export const [current_plugin, setCurrentPlugin] = [
    () => plugin_store[PluginStoreAttrs.current],
    (name: string | null) => {
        console.log(`setting new current => ${name}`)
        setPluginStore([PluginStoreAttrs.current], name)
    }
]

export const getPlugin = async (name: string | null) : Promise<Plugin> => {

    if (name == null) return Promise.reject('plugin name is null')

    const extension = name?.endsWith('.wasm') ? '' : '.wasm'

    const plugin_name = name?.endsWith('.wasm') ? name.replace('.wasm', '') : name

    //const stored_plugin = plugin_store[PluginStoreAttrs.plugins].get(plugin_name);

    return  await createPlugin({
        wasm: [{
            url: `http://localhost:8080/plugin-files/${name}${extension}`
        }]
    }, {
        useWasi : true,
        functions: {
            [EXTISM_USER] : {
                alert : (ctx: CallContext, msgAddr: bigint) => {
                    let msg = ctx.read(msgAddr)?.text()
                    return ctx.store(`${msg?.toUpperCase()}`)
                }
            }
        }
    }).then(plugin => {
        setPluginStore([PluginStoreAttrs.plugins], {...plugin_store[PluginStoreAttrs.plugins], [plugin_name]: plugin})
        return plugin
    })
}


