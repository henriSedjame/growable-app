import {Component, createEffect, createResource, createSignal} from 'solid-js';

import styles from './App.module.css';

import {createPlugin, Plugin} from "@extism/extism";

interface Person {
    age: number,
    name: string
}

const getPlugin = async () : Promise<Plugin> => {
    return await createPlugin({
        wasm: [{
            url: 'http://localhost:8080/plugins.wasm'
        }]
    }, {useWasi : true})
}


const App: Component = () => {
    const [message, setMessage] = createSignal("");

    const [plugin, {}] = createResource(getPlugin)

    const person : Person = {
        age: 29,
        name: "Joe"
    }

    createEffect(() => {
        plugin()?.call('hello', JSON.stringify(person))
            .then(m => setMessage(m?.text() ?? ''))
    });

    return (
        <div class={styles.App}>
            {message()}
        </div>
    );
};

export default App;
