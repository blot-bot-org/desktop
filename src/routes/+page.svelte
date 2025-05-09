<script lang="ts">
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import Dashboard from "$components/Dashboard.svelte";
    import Preview from "$components/Preview.svelte";
    import ClientWindow from "$components/ClientWindow.svelte";
    import Slider from "$components/parameters/Slider.svelte";

	import { fade } from "svelte/transition";
    import { quadInOut } from "svelte/easing";


    let previewRef;
    let dashboardRef;

    let showingModal = false;

    setTimeout(() => {
        console.log(previewRef);
    }, 500);


</script>

<main class="container">
    <div id="app">
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Kantumruy+Pro:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet">

        <Preview bind:this={previewRef} />

        {#if previewRef} <!-- used to defer loading until preview_ref is initialised -->
        <Dashboard bind:this={dashboardRef} onStateChange={(styleId, parameterObject) => { previewRef.gen_preview(styleId, parameterObject); }} printPressed={() => { showingModal = true; }} />
        {/if}
        
        {#if showingModal && previewRef}
            <div transition:fade={{ duration: 150, easing: quadInOut }}>
                <ClientWindow previewRef={previewRef} dashboardRef={dashboardRef} close={() => { showingModal = false; }} />
            </div>
        {/if}
    </div>
</main>

<style>
    /*
    * Master styling
    */

    * {
        margin: 0;
        padding: 0;

        font-family: "Kantumruy Pro", sans-serif;
        font-weight: 400;
    }

    .container {
        width: 100%;
        height: 100%;
    }

    #app {
        width: 100%;
        height: 100%;

        display: flex;
        overflow: none;
    }
</style>
