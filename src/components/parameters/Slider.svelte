<script lang="ts">
    export let min;
    export let max;
    export let name;
    export let id;

    export let value;
</script>

<div class="param-container slider-container">
    <label for={id}>{name}: {value}</label>
    <input class="slider-style" type="range" id={id} name={name} min={min} max={max} bind:value={value} />
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
    }

    .slider-style::-webkit-slider-runnable-track  {
        background: #D0D0D0;
        height: 10px;
    }

    .slider-style::-moz-range-track {
        background: #D0D0D0;
        height: 10px;
    }

    .slider-style::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;

        background: #000000;
        margin-top: -3px;
        height: 16px;
        width: 8px;
        border-radius: 3px;
    }


</style>
