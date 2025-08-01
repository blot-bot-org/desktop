<script lang="ts">
    import Tooltip from "$components/parameters/Tooltip.svelte"; 

    import { save, open } from "@tauri-apps/plugin-dialog";

    export let name;
    export let id;
    export let description;

    export let onChangeCallback;

    export let value;

    let labelElement;

    async function openFile() {
        let path = await open({
            multiple: false,
            directory: false,
        });

        if(path == null) {
            // user cancelled dialogue
            return;
        }

        value = path;

        if(onChangeCallback) {
            onChangeCallback(true);
        }
    }
</script>

<div class="param-container file-container">
    <label for={id} bind:this={labelElement}>{name}</label>
    <Tooltip element={labelElement} text={description} />
    <div>
        <button class="button-style" onclick={openFile}>Select File</button>
        <button class="button-style refresh" onclick={ () => { if(onChangeCallback) { onChangeCallback(false); } } }><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><path fill="currentColor" d="M17.91 14c-.478 2.833-2.943 5-5.91 5c-3.308 0-6-2.692-6-6s2.692-6 6-6h2.172l-2.086 2.086L13.5 10.5L18 6l-4.5-4.5l-1.414 1.414L14.172 5H12a8 8 0 0 0 0 16c4.079 0 7.438-3.055 7.931-7z"/></svg></button>
        <!-- the refresh button here calls a onChangeCallback to force an update -->
    </div>
</div>

<style>
    .file-container {
        color: var(--default-font);

        display: flex;
        align-items: center;
        justify-content: space-between;

        margin-left: 30px;
        margin-right: 30px;

        margin-bottom: 8px;
        margin-top: 8px;
    }

    .button-style {
        width: 100px;

        padding: 7px 8px 7px 8px;

        text-align: center;
        color: white;

        border: 1px solid var(--primary-selected);
        border-radius: 5px;

        background-color: var(--primary);
        box-shadow: 0px 5px 12px -1px #00000030;

        transition: 0.2s;
    }

    .button-style:hover {
        cursor: pointer;
        background-color: var(--primary-selected);
    }

    .refresh {
        width: auto;
        padding: 5px 5px 3px 5px;
        transform: translateY(3px);
    }
</style>

