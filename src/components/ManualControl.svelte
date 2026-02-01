<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    
    const props: {
        onClose(): void;
    } = $props();

    onMount(async () => {});

    let penLiftValue = $state(0);

    let gotoX = $state(0);
    let gotoY = $state(0);




    async function adjustControl(targetByte, value) {
        await invoke("apply_manual_control", { targetByte: targetByte, data: value });
    }

    async function goto() {
        gotoX = parseInt(gotoX);
        gotoY = parseInt(gotoY);
        await invoke("manual_goto", { x: gotoX, y: gotoY });
    }
</script>


<div class="fixed-background" />

<div class="window">

    <div class="main-container">
        <input class="slider" type="range" min="0" max="180" bind:value={penLiftValue} onchange={() => adjustControl(0x02, penLiftValue) }>
        <a>&nbsp;{penLiftValue}</a>

        <br>
        <br>

        <button onclick={() => adjustControl(0x03, 0x01) }>Enable Motors</button>
        <button onclick={() => adjustControl(0x03, 0x00) }>Disable Motors</button>

        <br>
        <br>

        <input class="number" min="0" max="300" step="0.1" bind:value={gotoX} />
        <input class="number" min="0" max="300" step="0.1" bind:value={gotoY} />
        <button onclick={() => goto() }>Invoke</button>
    </div>
    <button class="button close-button" onclick={() => { props.onClose(); }}>Close</button>

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





    .main-container {
        width: 100%;
        height: 100%;
    }

    .slider {
        width: 50%;
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

    .close-button {
        margin-left: 90%;
    }

</style>
