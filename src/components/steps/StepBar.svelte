<script lang="ts">
    import Step from "./Step.svelte";
    import { onMount } from "svelte";

    let mounted = false;
    onMount(() => { mounted = true; });

    // initialise and variables
    export let numBars: number;
    export let activeWidth: number;
    export let progress: number;
    export let isError;

    let refList = [];
    let remainingWidth = 100 - activeWidth;
    let defaultWidth = remainingWidth / (numBars - 1); // remainingWidth / remainingBars essentially

    $: {
        if(mounted) {
            isError; // update on error as well.
            if(progress >= 0) { // != -1
                setBarActive(progress);
            }
        }
    }

    export function setBarActive(idx: number) {
        // set all bars up to idx to blue, set all as short
        for(let i = 0; i < numBars; i++) {
            if(i <= idx) {
                refList[i].setColoured(true, isError);
            }
            refList[i].setWide(false);
        }

        // then set the one as wide
        refList[idx].setWide(true);
    }

</script>


<div id="step-bar">
    {#each [...Array(numBars).keys()] as i}
        <Step bind:this={refList[i]} activeWidth={activeWidth} defaultWidth={defaultWidth} id={`step-element-${i}`}  />
    {/each}
</div>


<style>
    #step-bar {
        width: 400px;
        height: 20px;

        display: flex;
    }
</style>
