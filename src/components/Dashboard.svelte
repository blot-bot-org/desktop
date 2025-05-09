<script lang="ts">
    import Parameters from "../configuration/parameters.json";
    import Slider from "$components/parameters/Slider.svelte";
    import Number from "$components/parameters/Number.svelte";
    import Text from "$components/parameters/Text.svelte";
    import FileSelector from "$components/parameters/FileSelector.svelte";
    import Divider from "$components/parameters/Divider.svelte";

    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import { listen } from '@tauri-apps/api/event';
    import { save, open } from "@tauri-apps/plugin-dialog";

    import { onMount } from "svelte";
    import toast, { Toaster } from "svelte-french-toast";

    const props: {
        printPressed(): void;
        pausePressed(): void;
        onStateChange(styleId: string, parameterObject: any): void;
    } = $props(); 

    const drawStyles = Object.keys(Parameters);
    const initialStyleId = drawStyles[0]; 

    let styleId = $state("");
    let styleName = $state("");
    let parameterObject = $state({});

    let drawingPaused = false;
    
    async function make_preview(event: Event) {
        if(event) event.preventDefault();

        props.onStateChange(styleId, parameterObject);
    }

    async function switchStyle(newStyleId) {
        parameterObject = {};

        styleId = newStyleId;
        styleName = Parameters[styleId]["name"];

        for(let object of Parameters[styleId]["parameters"]) {
            parameterObject[object.id] = object.default;
        }
        
        props.onStateChange(styleId, parameterObject);
    }

    async function print() {
        props.printPressed();
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

    async function saveFile() {
        let path = await save({
            filter: [
                {
                    name: "Blot Bot Draw",
                    extensions: ["bbd"]
                }
            ],
        });

        await invoke("save_file", { path: path, drawingId: styleId, jsonParams: JSON.stringify(parameterObject) });
        alert("file saved.");
    }

    async function openFile() {
        let path = await open({
            multiple: false,
            directory: false,
        });

        await invoke("open_file", { path: path })
            .then((val) => {
                styleId = val[0];
                parameterObject = JSON.parse(val[1]);

                props.onStateChange(styleId, parameterObject);
            })
            .catch((err) => {
                alert(err);
            });
    }

    switchStyle(initialStyleId);
</script>


<div id="dashboard">
    
    <div>

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
                <Slider min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {:else if param.type == "number"}
                <Number min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {:else if param.type == "text"}
                <Text maxlength={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {:else if param.type == "file_selector"}
                <FileSelector name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => make_preview(undefined)} />
            {/if}
            <Divider />
        {/each}
    </div>

    </div>
    
    <div>
        
        <div id="file-button-container">
            <button style="margin-right: 5px !important;" onclick={saveFile}>Save Drawing</button>
            <button style="margin-left: 5px !important;" onclick={openFile}>Open Drawing</button>
        </div>

        <button id="print-button" onclick={print} bind:this={button}>Print</button>

    </div>

    <Toaster />
</div>


<style>
    #dashboard {
        flex-grow: 1;
        height: 100%;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    #file-button-container {
        width: 100%;
        display: flex;
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
        background-color: #4681F4;
        border: 1px solid #2374F0;
        box-shadow: 0px 5px 12px -1px #00000030;
        color: white;
        margin: 0px 10px 10px 10px;
        border-radius: 5px;
        font-size: 1em;
        outline: none;

        transition: 0.2s;

        align-self: flex-end;
    }

    button:hover {
        cursor: pointer;
        background-color: #2374F0;
    }

    /*
        Must be applied to all parameter types (slider, number etc.)
    */
    :global(.param-container) {
        max-width: 100%;
        min-height: 40px;
    }
</style>
