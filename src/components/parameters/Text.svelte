<script lang="ts">
    export let maxlength;
    export let name;
    export let id;

    export let onChangeCallback;

    export let value;
</script>

<div class="param-container text-container">
    <label for={id}>{name}</label>
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
