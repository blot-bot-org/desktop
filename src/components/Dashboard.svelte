<script lang="ts">
    import Parameters from "../configuration/parameters.json";
    import Slider from "$components/parameters/Slider.svelte";
    import Number from "$components/parameters/Number.svelte";
    import Text from "$components/parameters/Text.svelte";
    import Divider from "$components/parameters/Divider.svelte";
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import { listen } from '@tauri-apps/api/event';

    import toast, { Toaster } from 'svelte-french-toast';



    export let previewRef;

    const drawStyles = Object.keys(Parameters);
    const initialStyleId = drawStyles[0]; 

    let styleId = "";
    let styleName = "";
    let parameterObject;

    let drawingPaused = false;
    
    async function make_preview(event: Event) {
        if(event) event.preventDefault();

        await previewRef.gen_preview(styleId, parameterObject);
    }

    async function switchStyle(newStyleId) {
        parameterObject = {};

        styleId = newStyleId;
        styleName = Parameters[styleId]["name"];

        for(let object of Parameters[styleId]["parameters"]) {
            parameterObject[object.id] = object.default;
        }
        
        let i = 0;
        while (previewRef == undefined) {
            await new Promise(r => setTimeout(r, 10));
            i += 1;

            if (i > 1000) { // 10 seconds
                console.log("Error! `previewRef` was never intialised in dashboard");
                break;
            }
        }

        previewRef.gen_preview(styleId, parameterObject);
    }

    async function print() {
        drawingPaused = false;
        
        let firmware_progress = await listen<string>("firm-prog", (ev) => { console.log(ev); handleToast(JSON.parse(ev["payload"])); });

        await invoke("send_to_firmware"); // this does not propogate errors, they're done through firm-prog channel

        firmware_progress();

        // can do this sorta thing
        /*
        invoke("send_to_firmware", {})
            .then((result) => console.log(result))
            .catch((err) => alert(err));
        */
    }

    async function pause() {
        document.getElementById("pause-button").classList.add("disabled-button");

        await invoke("pause_firmware", {});
        drawingPaused = !drawingPaused;
        await new Promise(r => setTimeout(r, 1000));

        document.getElementById("pause-button").classList.remove("disabled-button");
    }

    async function moveToStart() {
        await invoke("move_pen_to_start", {});
    }

    function handleToast(payload: any) {

        const toast_duration = 10E3 * 30;
            
        if(payload["event"] == "connection") {
            if(payload["error"]) {
                toast.error("Error: " + payload["message"], { position: "bottom-center", duration: toast_duration });
            } else {
                toast.success(payload["message"], { position: "bottom-center", duration: toast_duration });
            }
        } else if(payload["event"] == "drawing") {
            toast.success(payload["message"], { position: "bottom-center", duration: toast_duration });
        }
    }

    switchStyle(initialStyleId);
</script>


<div id="dashboard">
    
    <div class="style-container">
        <select bind:value={styleId} onchange={() => switchStyle(styleId)} >
            {#each drawStyles as dsid}
                <option value={dsid}>
                    {Parameters[dsid]["name"]}
                </option>
            {/each}
        </select>

    </div>

    <Divider />

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
    <button onclick={print}>Print</button>
    <button id="pause-button" onclick={pause}>{ drawingPaused ? "Resume" : "Pause" }</button>
    <button id="move-to-start" onclick={moveToStart}>Move To Start</button>

    <Toaster />
</div>


<style>
    #dashboard {
        flex-grow: 1;
        height: 100%;
    }

    .parameter-container, .style-container {
        margin: 20px;
    }


    :global(.disabled-button) {
        filter: brightness(70%) saturate(80%);
        pointer-events: none;
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
