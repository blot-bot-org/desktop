
<script lang="ts">

    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    
    const props: {
        onClose(): void;
    } = $props();


    let activeTab = $state("page");
    let possibleErrorText = $state("");

    let addressValue = $state("");
    let portValue = $state("");
    let motorInterspaceValue = $state("");
    let hpoValue = $state("");
    let vpoValue = $state("");
    let pageWidthValue = $state("");
    let pageHeightValue = $state("");


    // when the elements loads we load all the old values, if they exist
    onMount(async () => {
        await invoke("get_app_config")
            .then((val) => {
                let json = JSON.parse(val);
                addressValue = json.machine_addr;
                portValue = json.machine_port;
                motorInterspaceValue = json.phys_motor_interspace;
                hpoValue = json.phys_page_left_offset;
                vpoValue = json.phys_page_top_offset;
                pageWidthValue = json.phys_page_width;
                pageHeightValue = json.phys_page_height;
            })
            .catch((_) => {
                console.log("Error loading config, probably no config saved.");
            });
    });


    //
    // Usage: called when the user closes the settings dialogue.
    // It'll (annoyingly lol) stop the user closing the dialogue if any of the inputted data is wrong.
    // If the data is okay, the dialogue will close and the data will be written to the disk.
    //
    // Parameters: none
    // Returns: none
    //
    async function submit() {

        // first check that the values are valid
        if(addressValue == "" || addressValue.split(".").length != 4) {
            activeTab = "machine";
            possibleErrorText = "Error: Invalid address format.";

            return;
        }

        if(portValue == "" || portValue >= 65536 || portValue <= 1) {
            activeTab = "machine";
            possibleErrorText = "Error: Invalid port (1-65535).";

            return;
        }

        if(motorInterspaceValue == "" || motorInterspaceValue <= 0) {
            activeTab = "page";
            possibleErrorText = "Error: The motor interspace must be greater than 0";

            return;
        }

        if(hpoValue == "" || hpoValue <= 0) {
            activeTab = "page";
            possibleErrorText = "Error: The horizontal page offset must be greater than 0";

            return;
        }

        if(vpoValue == "" || vpoValue <= 0) {
            activeTab = "page";
            possibleErrorText = "Error: The vertical page offset must be greater than 0";

            return;
        }

        if(pageWidthValue == "" || pageWidthValue <= 0) {
            activeTab = "page";
            possibleErrorText = "Error: The page width must be greater than 0";

            return;
        }

        if(pageHeightValue == "" || pageHeightValue <= 0) {
            activeTab = "page";
            possibleErrorText = "Error: The page height must be greater than 0";

            return;
        }


        // then save
        await invoke("save_app_config", { stringifiedConfig: JSON.stringify(
            {
                "machine_addr":addressValue,
                "machine_port":portValue,

                "phys_motor_interspace":motorInterspaceValue,
                "phys_page_left_offset":hpoValue,
                "phys_page_top_offset":vpoValue,
                "phys_page_width":pageWidthValue,
                "phys_page_height":pageHeightValue
            }
        ) });

        props.onClose();
    }

</script>


<div class="fixed-background" />

<div class="window">

    <div class="tab-container">
        <button class="tab-button {activeTab == "machine" ? "active-tab" : ""}" onclick={() => {activeTab = "machine"; }}>Machine</button>
        <button class="tab-button {activeTab == "page" ? "active-tab" : ""}" onclick={() => {activeTab = "page"; }}>Page</button>
    </div>


    <div style="flex-grow: 1; display: flex; flex-direction: column;">
        
        {#if activeTab == "machine"}
            <div class="header-container"><a class="header-explain">Adjust the values to the IP and port of your machine</a></div>
            
            <div class="content-container">
                <div class="entry-container">
                    <div>
                        <label for="address">Address</label>
                        <input id="address" name="address" class="text-style" bind:value={addressValue} />
                    </div>
                    <div>
                        <label for="port">Port</label>
                        <input id="port" name="port" class="text-style" type="number" min="1" max="65535" bind:value={portValue} />
                    </div>
                </div>
            </div>
        {:else if activeTab == "page"}
            <div class="header-container"><a class="header-explain">Adjust the physical dimensions of your hardware and paper</a></div>

            <div class="content-container">
                <div class="entry-container">
                    <div>
                        <label for="motor-interspace">Motor Interspace</label>
                        <input id="motor-interspace" name="motor-interspace" class="text-style" type="number" min="1" max="10000" step="0.01" bind:value={motorInterspaceValue} />
                    </div>
                    <div>
                        <label for="hpo">Horizontal Page Offset</label>
                        <input id="hpo" name="hpo" class="text-style" type="number" min="0" max="10000" step="0.01" bind:value={hpoValue} />
                    </div>
                    <div>
                        <label for="vpo">Vertical Page Offset</label>
                        <input id="vpo" name="vpo" class="text-style" type="number" min="0" max="10000" step="0.01" bind:value={vpoValue} />
                    </div>
                    <div>
                        <label for="page-width">Page Width</label>
                        <input id="page-width" name="page-width" class="text-style" type="number" min="0" max="10000" step="0.01" bind:value={pageWidthValue} />
                    </div>
                    <div>
                        <label for="page-height">Page Height</label>
                        <input id="page-height" name="page-height" class="text-style" type="number" min="0" max="10000" step="0.01" bind:value={pageHeightValue} />
                    </div>
                </div>
            </div>
        {/if}
    </div>

    <div class="button-container">
        <a class="error-text">{possibleErrorText != "" ? "Error: " : ""}{possibleErrorText}</a>
        <button class="button" onclick={() => { submit(); } }>Save</button>
    </div>
</div>



<style>
    .window {
        position: fixed;

        background-color: var(--default-background);
        color: var(--default-font);
        width: 600px !important;
        height: 400px !important;
        display: block;
        border-radius: 5px;

        left: calc(50% - 300px);
        top: calc(50% - 200px);

        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);

        display: flex;
        flex-direction: column;

        padding: 40px;
    }





    .tab-container {
        width: 100%;
        height: 40px;

        display: flex;
        justify-content: center;
        gap: 15px;
    }

    .tab-button {
        color: var(--default-font);

        border: none;
        border-radius: 5px;
        background-color: transparent;

        height: 40px;
        min-width: 80px;

        transition: 0.25s;
    }

    .tab-button.active-tab {
        color: #3384F0;
        background-color: var(--divider);

        font-weight: 600;
    }

    .tab-button:not(.active-tab):hover {
        cursor: pointer;
        color: var(--primary);

        font-weight: 600;
    }



    .header-container {
        width: 100%;
        align-self: flex-start;
        margin-top: 5px;
    }

    .header-explain {
        width: 100%;
        text-align: center;

        width: 100%;

        color: var(--sub-font);
        font-size: 13px;
        font-weight: 500;
    }

    .content-container {
        flex-grow: 1;

        display: flex;
        flex-direction: column;

        justify-content: center;
        align-items: center;
    }

    .entry-container {
        display: flex;
        flex-direction: column;
        align-items: center;

        width: 100%;
        gap: 10px;
    }




    /* https://www.w3schools.com/howto/howto_css_hide_arrow_number.asp */
    input[type=number] {
        -moz-appearance: textfield;
    }
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }




    .text-style {
        background-color: var(--divider);
        color: var(--default-font);

        margin-left: 40px;
        margin-right: 10px;
        flex-grow: 1;

        border: none;
        border-bottom: 1px solid --var(divider);
        border-radius: 3px;
        box-shadow: 0px 5px 12px -6px #00000030;

        font-size: 0.94em;
        font-family: "Kantumruy Pro", sans-serif;

        height: 30px;
        width: 200px;
        outline: none;

        text-indent: 10px;
    }

    .button-container {
        display: flex;
        justify-content: space-between;

        height: 30px;
    }

    .button {
        background-color: var(--primary);
        border: 1px solid var(--primary-selected);
        padding: 5px 10px 6px 10px;

        border-radius: 5px;

        color: white;
        box-shadow: 0px 5px 12px -1px #00000030;

        cursor: pointer;
        transition: 0.25s background-color;
    }

    .button:hover {
        background-color: var(--primary-selected);
    }

    .error-text {
        color: red;

        font-size: 13px;
        font-weight: 500;

        margin-top: 6px;
    }
</style>
