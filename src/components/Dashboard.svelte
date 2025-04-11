<script lang="ts">
    import Parameters from "../configuration/parameters.json";
    import Slider from "$components/parameters/Slider.svelte";
    import Number from "$components/parameters/Number.svelte";
    import Text from "$components/parameters/Text.svelte";
    import Divider from "$components/parameters/Divider.svelte";

    export let previewRef;

    const initialStyleId = "lines";

    let styleId = "";
    let styleName = "";
    let parameterObject;
    
    async function make_preview(event: Event) {
        if(event) event.preventDefault();

        await previewRef.gen_preview(parameterObject);
    }

    function switchStyle(newStyle) {
        parameterObject = {};

        styleId = newStyle;
        styleName = Parameters[styleId]["name"];

        for(let object of Parameters[styleId]["parameters"]) {
            parameterObject[object.id] = object.default;
        }

        previewRef.gen_preview(parameterObject);
    }

    switchStyle(initialStyleId);
</script>


<div id="dashboard">
    <button on:click={make_preview}>Generate preview</button>
    <div class="parameter-container">
        {#each Parameters[styleId]["parameters"] as param}
            {#if param.type == "slider"}
                <Slider min={param.min} max={param.max} name={param.name} id={param.id} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {:else if param.type == "number"}
                <Number min={param.min} max={param.max} name={param.name} id={param.id} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {:else if param.type == "text"}
                <Text maxlength={param.max} name={param.name} id={param.id} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {/if}
            <Divider />
        {/each}
    </div>
</div>


<style>
    #dashboard {
        flex-grow: 1;
        height: 100%;
    }

    .parameter-container {
        margin: 20px;
    }

    button {
        width: calc(100% - 20px);
        height: 50px;
        text-align: center;
        border: none;
        background-color: #63c7e6;
        color: white;
        margin: 10px;
        border-radius: 5px;
        font-size: 1em;
        outline: none;

        transition: 0.2s;
    }

    button:hover {
        cursor: pointer;
        background-color: #53b7d6;
    }

    /*
        Must be applied to all parameter types (slider, number etc.)
    */
    :global(.param-container) {
        max-width: 100%;
        min-height: 40px;
    }
</style>
