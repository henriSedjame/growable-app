import {Plugin, PluginOutput} from "@extism/extism";

type PluginCallArg = string | number | Uint8Array | undefined

export const call = async <T>(plugin: Plugin, funcName: string, arg: T, argFn: (t: T) => PluginCallArg) : Promise<PluginOutput| null> => {
    return plugin.call(funcName, argFn(arg))
}