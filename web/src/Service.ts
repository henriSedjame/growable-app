export const storePlugin = async (file: File | undefined, onsuccess: () => Promise<void>) => {

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


export const deletePlugin = async  (name: String) => {
    const plugin_name = name?.endsWith('.wasm') ? name.replace('.wasm', '') : name
    return await fetch(`http://localhost:8080/plugins?name=${plugin_name}`, {
        method: 'delete',
    }).then(response => response.json())
        .then(data => data as boolean)
}