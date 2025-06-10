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
            onChangeCallback();
        }
    }
</script>

<div class="param-container file-container">
    <label for={id} bind:this={labelElement}>{name}</label>
    <Tooltip element={labelElement} text={description} />
    <button class="button-style" onclick={openFile}>Select File</button>
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
</style>

