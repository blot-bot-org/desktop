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

    import Icon from "@iconify/svelte";

    const props: {
        printPressed(): void;
        pausePressed(): void;
        onStateChange(styleId: string, parameterObject: any): void;
        onAppConfigOpen(): void;
    } = $props(); 

    const drawStyles = Object.keys(Parameters);
    const initialStyleId = drawStyles[0]; 

    let styleId = $state("");
    let parameterObject = $state({});

    let customParametersFile = $state({}); // holds the same stuff as the parameters.json file

    //
    // Usage: called to regenerate the preview, usually because a parameter in the dashboard has updated.
    //
    // Parameters: none
    // Returns: none
    //
    export async function makePreview(event: Event) {
        if(event) event.preventDefault();

        await props.onStateChange(styleId, parameterObject);

    }

    //
    // Usage: loads custom parameters to populate the dashboard when a plugin is loaded.
    // The function will preserve pre-existing parameter values if they exist, add new ones
    // with their default value and remove any old parameters.
    //
    // Parameters: newFile, false is the plugin has been reloaded
    // Returns: none
    //
    async function loadCustomParameters(newFile: bool, existingParameters: any | undefined) {

        // this lazy loads all parameters if newFile is not new
        // it preserves any pre-existing parameter values
        // it removes any old parameters which have been removed
        // it adds any new parameters which have been added in
        await invoke("get_parameters", { path: parameterObject["plugin_path"] })
            .then(async (val) => {

                // load parameters as json, including filename etc
                let json = JSON.parse(val);
                customParametersFile = json;


                // these are the keys to keep
                let currentKeys = Object.keys(parameterObject);
                let newKeys = json["parameters"].map(p => p.id);

                // delete all keys which arent in new obj, excluding plugin path and parameters
                for(let key in parameterObject) {
                    if (!newKeys.includes(key) && key != "plugin_path" && key != "plugin_parameters_json") {
                        delete parameterObject[key];
                    }
                }

                console.log("EXISTING PARAMS: ")
                console.log(existingParameters)
                for(let object of customParametersFile["parameters"]) {
                    if(!currentKeys.includes(object.id) || newFile) {
                        if(existingParameters && Object.keys(existingParameters).includes(object.id)) {
                            parameterObject[object.id] = existingParameters[object.id];
                        } else {
                            parameterObject[object.id] = object.default;
                        }
                    }
                }

            })
            .catch((err) => {
                toast.error(`Error getting plugin parameters! ${err}`, { position: "bottom-center", duration: 3000 });
            });

    }


    //
    // Usage: used to switch the style to a new one, given a style ID. It then regenerates the preview.
    //
    // Parameters: the new style ID
    // Returns: none
    //
    async function switchStyle(newStyleId) {
        parameterObject = {};
        customParametersFile = {};

        styleId = newStyleId;

        for(let object of Parameters[styleId]["parameters"]) {
            parameterObject[object.id] = object.default;
        }

        
        await props.onStateChange(styleId, parameterObject);
    }

    //
    // Usage: calls the current preview to be printed
    //
    // Parameters: none
    // Returns: none
    //
    async function print() {
        props.printPressed();
    }

    //
    // Usage: opens an OS file saver dialogue, saves the current drawing parameters and style to a file, and handles plugins separately.
    //
    // Parameters: none
    // Returns: none
    //
    async function saveFile() {
        let path = await save({
            filter: [
                {
                    name: "Blot Bot Draw",
                    extensions: ["bbd"]
                }
            ],
        });

        if(path == null) {
            // user cancelled the dialog menu
            return;
        }

        let jsonParams;
        if(styleId == "custom") {
            jsonParams = JSON.stringify({"plugin_path":parameterObject["plugin_path"], "plugin_parameters_json":JSON.stringify(parameterObject)});
        } else {
            jsonParams = JSON.stringify(parameterObject);
        }

        await invoke("save_file", { path: path, drawingId: styleId, jsonParams: jsonParams });
        toast.success("File saved!", { position: "bottom-center", duration: 3000 });
    }

    //
    // Usage: opens an OS file selector dialogue, attempts to load a plugin from disk, and handles plugins separately.
    //
    // Parameters: none
    // Returns: none
    //
    async function openFile() {
        let path = await open({
            multiple: false,
            directory: false,
        });

        if(path == null) {
            // user cancelled the dialog menu
            return;
        }

        await invoke("open_file", { path: path })
            .then(async (val) => {
                styleId = val[0];

                if(styleId == "custom") {

                    let json = JSON.parse(val[1]);
                    parameterObject = JSON.parse(json["plugin_parameters_json"]);

                    loadCustomParameters(true, parameterObject);

                    await props.onStateChange(styleId, parameterObject);

                } else {
                    parameterObject = JSON.parse(val[1]);

                    await props.onStateChange(styleId, parameterObject);
                }

            })
            .catch((err) => {
                toast.error(`Error opening file! ${err}`, { position: "bottom-center", duration: 3000 });
            });
    }

    switchStyle(initialStyleId);
</script>


<div id="dashboard">
    
    <div style="flex-grow: 1; margin-bottom: 10px; overflow: auto;">

    <div class="style-container">
        Draw style:
        <select id="style-selector" bind:value={styleId} onchange={() => switchStyle(styleId)} >
            {#each drawStyles as dsid}
                <option value={dsid}>
                    {Parameters[dsid]["name"]}
                </option>
            {/each}
        </select>

    </div>


    <div class="parameter-container">
        <Divider />

        {#if styleId == "custom"}

            <FileSelector name={"Plugin File"} id={"plugin_path"} description={"The path to the Python file"} bind:value={parameterObject["plugin_path"]} onChangeCallback={async (newFile) => { await loadCustomParameters(newFile); await makePreview(undefined); } } />

            {#if parameterObject.length != 2}
                <Divider />
            {/if}

            {#each customParametersFile["parameters"] as param}
                {#if !param.name.startsWith(".")} <!-- ignore hidden parameters (name starts with a .) -->

                    {#if param.type == "slider"}
                        <Slider min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "number"}
                        <Number min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "text"}
                        <Text maxlength={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "file_selector"}
                        <FileSelector name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {/if}

                    <Divider />
                
                {/if}
            {/each} 

        {:else}

            {#each Parameters[styleId]["parameters"] as param}
                {#if !param.name.startsWith(".")} <!-- ignore hidden parameters (name starts with a .) -->

                    {#if param.type == "slider"}
                        <Slider min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "number"}
                        <Number min={param.min} max={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "text"}
                        <Text maxlength={param.max} name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {:else if param.type == "file_selector"}
                        <FileSelector name={param.name} id={param.id} description={param.description} bind:value={parameterObject[param.id]} onChangeCallback={() => makePreview(undefined)} />
                    {/if}

                    <Divider />
                
                {/if}
            {/each}

        {/if}
    </div>

    </div>
    
    <div>
        
        <div class="button-container">
            <button style="margin-right: 5px !important;" onclick={saveFile}>Save Drawing</button>
            <button style="margin-left: 5px !important;" onclick={openFile}>Open Drawing</button>
        </div>
        

        <div class="button-container">
            <button style="margin-right: 5px !important;" id="print-button" onclick={print}>Print</button>
            <button style="margin-left: 5px !important;" id="print-config-button" onclick={props.onAppConfigOpen}><Icon style="transform: translateY(1px);" icon="material-symbols:settings" width="24" height="24" /></button>
        </div>

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

        overflow: hidden;

        background-color: var(--default-background);
    }

    .button-container {
        width: 100%;
        display: flex;
    }

    #print-config-button {
        width: 60px;
    }

    .parameter-container {
        margin: 20px;

        overflow: scroll;
        flex-grow: 1;
    }

    .style-container {
        color: var(--default-font);

        display: flex;
        align-items: center;

        margin: 30px 48px 30px 48px;
        gap: 15px;
    }

    #style-selector {
        border-radius: 5px;
        box-shadow: 0px 5px 12px -1px #00000030;
        outline: none;

        border: 1px solid var(--divider);
        min-width: 140px;
        min-height: 40px;

        border-radius: 5px;
    }


    :global(.disabled-button) {
        filter: brightness(70%) saturate(80%);
        pointer-events: none;
    }

    button {
        width: calc(100% - 20px);
        height: 50px;
        margin: 0px 10px 10px 10px;

        text-align: center;
        color: white;
        font-size: 1em;

        border: 1px solid var(--primary-selected);
        border-radius: 5px;
        background-color: var(--primary);
        box-shadow: 0px 5px 12px -1px #00000030;
        outline: none;

        transition: 0.2s;

        align-self: flex-end;
    }

    button:hover {
        cursor: pointer;
        background-color: var(--primary-selected);
    }

    /*
        Must be applied to all parameter types (slider, number etc.)
    */
    :global(.param-container) {
        max-width: 100%;
        min-height: 40px;
    }
</style>
