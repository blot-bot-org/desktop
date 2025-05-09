<script lang="ts">
    import Tooltip from "$components/parameters/Tooltip.svelte";

    export let maxlength;
    export let name;
    export let id;
    export let description;

    export let onChangeCallback;

    export let value;

    let labelElement;
</script>

<div class="param-container text-container">
    <label for={id} bind:this={labelElement}>{name}</label>
    <Tooltip element={labelElement} text={description} />
    <input class="text-style" type="text" id={"param-" + id} name={name} maxlength={maxlength} bind:value={value} on:change={() => { if(onChangeCallback) { onChangeCallback() }; }} />
</div>

<style>
    .text-container {
        display: flex;
        flex-direction: column;
        margin-left: 30px;
        margin-right: 30px;

        margin-bottom: 12px;
    }

    .text-style {
        max-width: 300px;
        width: 70%;
    }
</style>
