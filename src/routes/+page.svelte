<script lang="ts">
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import Dashboard from "$components/Dashboard.svelte";
    import Preview from "$components/Preview.svelte";
    import Slider from "$components/parameters/Slider.svelte";

    let previewRef;
    let dashboardRef;
</script>

<main class="container">
    <div id="app">
        <Preview bind:this={previewRef} />
        <Dashboard bind:this={dashboardRef} previewRef={previewRef} />
    </div>
</main>

<style>
    /*
    * Master styling
    */

    * {
        margin: 0;
        padding: 0;
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
