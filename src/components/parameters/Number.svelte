<script lang="ts">
    import Tooltip from "$components/parameters/Tooltip.svelte";

    export let min;
    export let max;
    export let name;
    export let id;
    export let description;

    export let onChangeCallback;

    export let value;

    let labelElement;
</script>

<div class="param-container number-container">
    <label for={id} bind:this={labelElement}>{name}</label>
    <Tooltip element={labelElement} text={description} />
    <input class="number-style" type="number" id={"param-" + id} name={name} min={min} max={max} bind:value={value} on:change={() => { if(onChangeCallback) { onChangeCallback() }; }} />
</div>

<style>
    .number-container {
        color: var(--default-font);
    
        display: flex;
        margin: 6px 30px 6px 30px;

        align-items: center;
        justify-content: space-between;
    }

    .number-style {
        max-width: 300px;
        width: 30%;
        height: 28px;

        border: none;
        border-radius: 4px;
        text-align: center;

        background-color: var(--divider);
        color: var(--default-font);
    }
</style>
