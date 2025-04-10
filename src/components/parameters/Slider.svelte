<script lang="ts">
    export let min;
    export let max;
    export let name;
    export let id;

    export let value;
</script>

<div class="slider-container">
    <input type="range" id={id} name={name} min={min} max={max} bind:value={value} />
    <label for={id}>{name}</label>
</div>
