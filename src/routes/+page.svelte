<script lang="ts">
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import Dashboard from "$components/Dashboard.svelte";
    import Preview from "$components/Preview.svelte";
    import ClientWindow from "$components/ClientWindow.svelte";
    import AppConfig from "$components/AppConfig.svelte";

	import { fade } from "svelte/transition";
    import { quadInOut } from "svelte/easing";


    let previewRef;
    let dashboardRef;

    let showingModal = false;
    let showingAppConfig = false;

    setTimeout(() => {
        // document.documentElement.setAttribute("data-theme", "dark");
    }, 500);


</script>

<main class="container">
    <div id="app">
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Kantumruy+Pro:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet">

        <Preview bind:this={previewRef} />

        {#if previewRef} <!-- used to defer loading until preview_ref is initialised -->
        <Dashboard bind:this={dashboardRef} onStateChange={(styleId, parameterObject) => { previewRef.gen_preview(styleId, parameterObject); }} printPressed={() => { showingModal = true; }} onAppConfigOpen={() => { showingAppConfig = true; console.log("hi"); }} />
        {/if}
        
        {#if showingModal && previewRef}
            <div transition:fade={{ duration: 150, easing: quadInOut }}>
                <ClientWindow previewRef={previewRef} dashboardRef={dashboardRef} close={() => { showingModal = false; }} />
            </div>
        {/if}

        {#if showingAppConfig}
            <div transition:fade={{ duration: 150, easing: quadInOut }}>
                <AppConfig onClose={() => { showingAppConfig = false; dashboardRef.make_preview(); }} />
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

    /* used by a few popup windows */
    :global(.fixed-background) {
        position: fixed;

        top: 0;
        left: 0;

        width: 100% !important;
        height: 100% !important;

        background-color: #000000c5;
    }

    :global(:root) {
        --default-background: #fff;
        --preview-background: #ddd;
        --divider: #f0f0f0;

        --primary: #4681F4;
        --primary-selected: #2374F0;
        --error: #aaa;

        --default-font: #101010;
        --sub-font: #404040;
    }

    :global([data-theme="dark"]) {
        --default-background: #323232;
        --preview-background: #252525;
        --divider: #555555;

        --primary: #4681F4;
        --primary-selected: #2374F0;
        --error: #aaa;

        --default-font: #e8e8e8;
        --sub-font: #a0a0a0;
    }
</style>
