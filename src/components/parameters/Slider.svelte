<script lang="ts">
    export let min;
    export let max;
    export let name;
    export let id;

    export let onChangeCallback;

    export let value;
</script>

<div class="param-container slider-container">
    <label for={id}>{name}: {value}</label>
    <input class="slider-style" type="range" id={"param-" + id} name={name} min={min} max={max} bind:value={value} on:change={() => { if(onChangeCallback) { onChangeCallback() }; }} />
</div>

<style>
    .slider-container {
        display: flex;
        flex-direction: column;
        margin-left: 30px;
        margin-right: 30px;

        margin-bottom: 12px;
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
        background: #D0D0D0;
        height: 2px;

        border-radius: 3px;
    }

    .slider-style::-moz-range-track {
        background: #D0D0D0;
        height: 2px;

        border-radius: 3px;
    }

    .slider-style::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;

        background: #000000;
        margin-top: -7px;
        height: 16px;
        width: 5px;
        border-radius: 3px;
    }


</style>
