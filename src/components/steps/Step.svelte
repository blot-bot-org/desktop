<script lang="ts">
    import { onMount } from "svelte";

    let mounted = false;

    export let defaultWidth: number;
    export let activeWidth: number;
    export let id: number;

    let selfElement = undefined;

    export function setWide(state: bool): void {
        selfElement.style.width = `${(state ? activeWidth : defaultWidth)}%`;
    }

    // gives a bar a colour if state true, if err false bar primary accent, if err true, bar error colour
    export function setColoured(state: bool, err: bool): void {
        if(state) {
            selfElement.classList.remove("bar-colour-" + (!err ? "in" : "") + "valid");
            selfElement.classList.add("bar-colour-" + (err ? "in" : "") + "valid");
        } else {
            selfElement.classList.remove("bar-colour-invalid");
            selfElement.classList.remove("bar-colour-valid");
        }

    }

    onMount(async () => {
        selfElement = document.getElementById(id);
        requestAnimationFrame(() => { selfElement.style.transition = "0.2s ease"; }); // set animation after one frame
    });

</script>

<div class="bar" id={id}>
</div>

<style>
    .bar {
        width: 30px;
        height: 11px;
        background-color: var(--preview-background);
        border-radius: 8px;
        border: 3px solid white; /* whatever the bg is */
        box-sizing: border-box;
    }

    :global(.bar-colour-valid) {
        background-color: var(--primary) !important;
    }

    :global(.bar-colour-invalid) {
        background-color: var(--error) !important;
    }
</style>
