import {Component,  createMemo, createSignal, Show} from 'solid-js';

import styles from './App.module.css';
import {plugin, refetch} from "./State";

interface Person {
    age: number,
    name: string
}

const person: Person = {
    age: 29,
    name: "Joe"
}

const saveFile = async (file: File | undefined, onsuccess: () => void) => {

    if (file) {
        const formData: FormData = new FormData()

        formData.append('file', file)

        await fetch('http://localhost:8080/plugins', {
            method: 'POST',
            body: formData
        }).then(r => r.json())
            .then(data => {
                console.log(data);
                onsuccess()
            })
    }

}

const App: Component = () => {

    const [message, setMessage] = createSignal("");

    const [files, setFiles] = createSignal<File>()

    // @ts-ignore
    let fileInputRef: HTMLInputElement = null;

    createMemo(() => {
        plugin()?.getExports().then(exports => {
            exports.forEach(ex => {
                console.log(ex.name, " =>" , ex.kind)

            })
        });

        plugin()?.call('hello', JSON.stringify(person))
            .then(m => setMessage(m?.text() ?? ''))
    });

    function handleFile(e: Event) {
        let fileList = ((e as InputEvent).target as HTMLInputElement).files;
        console.log(fileList);
        if (fileList) {
            let file = fileList.item(0);
            if (file) {
                setFiles(file)
            }
        }
    }

    const save = () => {
        saveFile(files(), () => {
            refetch()
            setFiles(undefined)
            fileInputRef.value = ''
        })
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
