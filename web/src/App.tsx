import {Component, createEffect, createSignal} from 'solid-js';

import styles from './App.module.css';
import {plugin} from "./State";

interface Person {
    age: number,
    name: string
}


const App: Component = () => {

    const [message, setMessage] = createSignal("");

    const person : Person = {
        age: 29,
        name: "Joe"
    }

    createEffect(() => {
        plugin()?.getExports().then(exports => {
            exports.forEach(ex => {
                console.log(ex.name, " =>" , ex.kind)

            })
        });

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
