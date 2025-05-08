<script lang="ts">
    import { save, open } from "@tauri-apps/plugin-dialog";

    export let name;
    export let id;

    export let onChangeCallback;

    export let value;

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
    <label for={id}>{name}</label>
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

