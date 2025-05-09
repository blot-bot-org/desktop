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
        display: flex;
        align-items: center;
        justify-content: space-between;

        margin-left: 30px;
        margin-right: 30px;

        margin-bottom: 12px;
    }

    .button-style {
        max-width: 300px;
        width: 30%;

        padding: 5px 8px 5px 8px;

        text-align: center;
        color: white;

        border: 1px solid #2374F0;
        border-radius: 5px;

        background-color: #4681F4;
        box-shadow: 0px 5px 12px -1px #00000030;

        transition: 0.2s;
    }

    .button-style:hover {
        cursor: pointer;
        background-color: #2374F0;
    }
</style>

