
<script lang="ts">

    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    
    const props: {
        onClose(): void;
    } = $props();


    onMount(async () => {
        let cachedValue = await invoke("get_machine_config");

        if(cachedValue == "null") {
            return;
        }

        let parts = cachedValue.split(":");
        document.getElementById("address").value = parts[0];
        document.getElementById("port").value = parts[1].substring(0, parts[1].length);
    });


    async function submit() {
        await invoke("save_machine_config", { addr: document.getElementById("address").value, port: Math.max(1, Math.min(65535, parseInt(document.getElementById("port").value))) });

        props.onClose();
    }
</script>


<div class="fixed-background" />

<div class="window">
    <div class="header-container"><a class="header">Machine Configuration</a><a class="header-explain">Adjust the values to the IP and port of your machine</a></div>

    <div class="entry-container">
        <div>
            <label for="address">Address</label>
            <input id="address" name="address" class="text-style" />
        </div>
        <div>
            <label for="port">Port</label>
            <input id="port" name="port" class="text-style" type="number" min="1" max="65535" />
        </div>
    </div>

    <div class="button-container">
        <button class="button" onclick={() => { submit(); } }>Done</button>
    </div>
</div>



<style>
    .window {
        position: fixed;

        background-color: #FFFFFF;
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

    .header-container {
        display: flex;
        flex-direction: column;
        justify-items: center;
        align-items: center;

        width: 100%;

        margin-top: 10px;
        margin-bottom: 120px;
    }

    .header {
        width: 100%;
        text-align: center;

        font-weight: 600;
        font-size: 1.1em;
    }

    .header-explain {
        width: 100%;
        text-align: center;

        font-weight: 400;
        font-size: 0.9em;
    }

    .entry-container {
        display: flex;
        flex-direction: column;
        align-items: center;

        width: 100%;
        gap: 10px;

        flex-grow: 1;
    }

    .text-style {
        margin-left: 40px;
        margin-right: 10px;
        flex-grow: 1;

        border: none;
        border-bottom: 1px solid #D0D0D0;
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
        justify-content: flex-end;

        height: 30px;
    }

    .button {
        background-color: #3384F0;
        border: 1px solid #2374F0;
        padding: 5px 10px 6px 10px;

        border-radius: 5px;

        color: white;
        box-shadow: 0px 5px 12px -1px #00000030;

        cursor: pointer;
        transition: 0.25s background-color;
    }

    .button:hover {
        background-color: #2374F0;
    }
</style>
