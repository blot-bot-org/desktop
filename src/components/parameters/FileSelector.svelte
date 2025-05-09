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

        value = path;

        if(onChangeCallback) {
            onChangeCallback();
        }
    }
</script>

<div class="param-container number-container">
    <label for={id} bind:this={labelElement}>{name}</label>
    <Tooltip element={labelElement} text={description} />
    <button onclick={openFile}>Select File</button>
</div>

<style>
    .number-container {
        display: flex;
        margin-left: 30px;
        margin-right: 30px;

        margin-bottom: 12px;
    }

    .number-style {
        max-width: 300px;
        width: 30%;
    }
</style>

