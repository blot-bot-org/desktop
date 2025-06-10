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

<div class="param-container slider-container">
    <label for={id} bind:this={labelElement}>{name}: {value}</label>
    <Tooltip element={labelElement} text={description} />
    <input class="slider-style" type="range" id={"param-" + id} name={name} min={min} max={max} bind:value={value} on:change={() => { if(onChangeCallback) { onChangeCallback() }; }} />
</div>

<style>
    .slider-container {
        color: var(--default-font);

        display: flex;
        align-items: center;
        justify-content: space-between;

        margin: 6px 30px 6px 30px;
    }

    .slider-style {
        max-width: 300px;
        width: 70%;

        -webkit-appearance: none;
        background: transparent;
        cursor: pointer;

        margin-top: 8px;
    }

    .slider-style::-webkit-slider-runnable-track  {
        background: var(--divider);
        height: 2px;

        border-radius: 3px;
    }

    .slider-style::-moz-range-track {
        background: var(--preview-background);
        height: 2px;

        border-radius: 3px;
    }

    .slider-style::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;

        background: var(--default-font);
        margin-top: -7px;
        height: 16px;
        width: 6px;
        border-radius: 3px;

        transition: 0.3s;
    }

    .slider-style::-webkit-slider-thumb:hover {
        background: var(--default-font);
    }


</style>
