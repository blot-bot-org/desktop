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
        align-items: center;
        justify-content: space-between;
        
        margin: 8px 30px 4px 30px;
    }

    .text-style {
        margin-left: 40px;
        margin-right: 10px;
        flex-grow: 1;

        border: none;
        border-bottom: 1px solid var(--primary);
        box-shadow: 0px 5px 12px -3px #00000030;

    }
</style>
