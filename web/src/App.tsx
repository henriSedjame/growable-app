import {Component, createEffect, createMemo, createResource, createSignal, Show} from 'solid-js';

import styles from './App.module.css';
import {current_plugin, getPlugin,  setCurrentPlugin} from "./State";
import {call} from "./utils";
import {deletePlugin, storePlugin} from "./Service";

interface Person {
    age: number,
    name: string
}

const person: Person = {
    age: 29,
    name: "Joe"
}


const App: Component = () => {

    const [plugin, { refetch: refetchPlugin }] = createResource(current_plugin, getPlugin)

    const [message, setMessage] = createSignal("");

    const [files, setFiles] = createSignal<File>()

    // @ts-ignore
    let fileInputRef: HTMLInputElement = null;

    createEffect( () => {
        const plug = plugin()
        if (plug) {
            call(plug, 'hello', person, JSON.stringify)
                .then(m => setMessage(m?.text() ?? ''));

            plug.getImports().then(is => {
                is.forEach(i => console.log(i.name))
            })

        }

    });

    function handleFile(e: Event) {
        let fileList = ((e as InputEvent).target as HTMLInputElement).files;
        if (fileList) {
            let file = fileList.item(0);
            if (file) {
                setFiles(file)
            }
        }
    }

    const save = async () => {
        let file = files();
        if (file) {
            await deletePlugin(file.name).then(async deleted => {
                if (deleted) {
                    await storePlugin(file, async () => {
                        setCurrentPlugin(file?.name ?? null)
                        refetchPlugin()
                        setFiles(undefined)
                        fileInputRef.value = ''
                    })
                }
            })
        }
    }

    return (
        <div class={styles.App}>
            <p>{message()}</p>
            <input
                type="file"
                accept=".wasm"
                ref={fileInputRef}
                onChange={handleFile}
                textContent="Pick a file"
            />
            <Show when={files()}>
                <button onClick={save}> Send
                </button>
            </Show>

        </div>
    );
};

export default App;
