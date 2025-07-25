<script lang="ts">
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition"
	import { Pulse } from 'svelte-loading-spinners';
    import toast, { Toaster } from "svelte-french-toast";


    let imageWidth = $state(210);
    let imageHeight = $state(297);

    let estDrawTimeSeconds = $state(-1);
    let instructionByteNumber = $state(0);

    (async () => {
        const unlisten = await getCurrentWindow().onResized(({ payload: size }) => {
            recomputeImageResolution(imageWidth, imageHeight);
        });
    })()

    // calculates the on-screen image resolution
    function recomputeImageResolution() {
        // get the real image resolution - so the new url needs to be set by here
        let imageElement = document.getElementById("preview-image");
        let realImgWidth = imageElement.naturalWidth;
        let realImgHeight = imageElement.naturalHeight;

        let previewStageElement = document.getElementById("preview-image-container");

        let marginRatio = 0.1;

        // these values have pre-calculated margin
        let stageWidth = previewStageElement.getBoundingClientRect().width * (1 - (2 * marginRatio));
        let stageHeight = previewStageElement.getBoundingClientRect().height * (1 - (2 * marginRatio));

        // calculate dimension to maximise
        let ratio = realImgWidth / realImgHeight;
        let maxWidth = true; // this if statement can be simplified
        if (stageWidth > ratio * stageHeight) {
            maxWidth = false;
        }

        // mod global states here
        if(maxWidth) {
            imageWidth = Math.round(stageWidth);
            imageHeight = Math.round(stageWidth / ratio);
        } else {
            imageWidth = Math.round(stageHeight * ratio);
            imageHeight = Math.round(stageHeight);
        }
    }


    let imageSrc = $state("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANIAAAEpAQMAAADcde5vAAAAAXNSR0IB2cksfwAAAAlwSFlzAAALEwAACxMBAJqcGAAAAANQTFRF////p8QbyAAAAB9JREFUGBntwTEBAAAAwiD7p14IX2AAAAAAAAAAAIcAIHwAAU/BTAIAAAAASUVORK5CYII=");
    let renderLoadingOverlay = $state(true);
  
    export async function gen_preview(style_id, parameter_object) {
        if(style_id == undefined || parameter_object == undefined) {
            console.log("Could not generate preview as style or parameters were undefined.");
            return;
        }

        let timeoutRef = setTimeout(() => {
            renderLoadingOverlay = true;
        }, 250);

        let path = await invoke("gen_preview", { styleId: style_id, jsonParams: JSON.stringify(parameter_object) });
        
        if(path.startsWith("error")) {
            toast.error(`Error generating preview! ${path.split(":")[1]}`, { position: "bottom-center", duration: 3000 });
            clearTimeout(timeoutRef);

            estDrawTimeSeconds = -1;
            instructionByteNumber = 0;

            return;
        }

        await updateTime();

        let imageUrl = convertFileSrc(path);
        clearTimeout(timeoutRef);
        renderLoadingOverlay = false;
  
        imageUrl += `?refresh=${Math.floor(Math.random() * 10000000)}`; // hack to force reload image by adding random parameter to change url
        imageSrc = imageUrl;
        
        setTimeout(() => {
            recomputeImageResolution();
        }, 50); // kinda bad, but only every visually bugs when page dimensions change... how to wait for image to finish loading?
    }
  
    export function getImageUrl(): string {
        return imageSrc;
    }

    onMount(() => {
        recomputeImageResolution(); // resize the initial blank image
    });

    async function updateTime() {
        await invoke("get_image_stats")
            .then((out) => { // out[0] = time in seconds, out[1] = num of bytes
                if(out[0] == 0) {
                    estDrawTimeSeconds = -1;
                }
                estDrawTimeSeconds = out[0];
                instructionByteNumber = out[1];
            })
            .catch((err) => {
                estDrawTimeSeconds = -1;
            });
    }

    function formatTime(timeSeconds) {
        // we'll build it backwards, adding components as needed.

        let time = (timeSeconds % 60).toString() + "s";
        if(timeSeconds > 60) {
            time = (Math.floor((timeSeconds / 60) % 60)).toString() + "m " + time;
        }

        if(timeSeconds > 60 * 60) {
            time = (Math.floor(timeSeconds / 60 / 60)).toString() + "h " + time;
        }
        
        return time;
    }
</script>

<div id="preview-container">
    <div id="preview-image-container">
        <div style="width: {imageWidth}px; height: {imageHeight}px;" >
            <img id="preview-image" src={imageSrc} style="width: {imageWidth}px; height: {imageHeight}px;" />
            {#if renderLoadingOverlay}
                <div class="preview-overlay" style="width: {imageWidth}px; height: {imageHeight}px;" in:fade={{ duration: 250 }}>
                    <Pulse size="34" color="#EEEEEE" unit="px" duration="1s" />
                </div>
            {/if}
        </div>
    </div>

    <div id="preview-stat-container">
        <a class="stat-title stat-text">Image stats:</a>
        <a class="stat-text">Instruction bytes: {(instructionByteNumber > 0 ? instructionByteNumber : "none")}</a>
        <a class="stat-text">Est. drawing time: {#if estDrawTimeSeconds >= 0}{formatTime(estDrawTimeSeconds)} @ 500 s/s{:else}unknown{/if}</a>
    </div>
</div>

<style>
    #preview-container {
        background-color: var(--preview-background);
        height: 100%;
        width: 65% !important; /* master width */

        box-shadow: inset -68px 0px 18px -67px rgba(0,0,0,0.08);

        box-sizing: border-box;
        border-right: 1px solid var(--divider);
    }

    #preview-image-container {
        height: 100%;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    #preview-image {
        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);
        position: absolute;
    }

    
    /* used by error + loading overlays */
    .preview-overlay {
        background-color: #00000070;
        position: absolute;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    #preview-stat-container {
        background-color: #00000050;
        position: fixed;
        bottom: 0;
        left: 0;

        padding: 6px;
        border-top-right-radius: 6px;

        display: flex;
        flex-direction: column;
    }

    .stat-text {
        color: #ffffffaa;
    }

    .stat-title {
        text-transform: uppercase;
        opacity: 0.8;
        font-weight: 600;
        font-size: 0.8em;
        letter-spacing: 0.4px;
    }


</style>
