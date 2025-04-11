<script lang="ts">
    export let min;
    export let max;
    export let name;
    export let id;

    export let onChangeCallback;

    export let value;
</script>

<div class="param-container number-container">
    <label for={id}>{name}</label>
    <input class="number-style" type="number" id={"param-" + id} name={name} min={min} max={max} bind:value={value} on:change={() => { if(onChangeCallback) { onChangeCallback() }; }} />
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
