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

    export function setColoured(state: string): void {
        if(state) {
            selfElement.classList.add("bar-colour");
        } else {
            selfElement.classList.remove("bar-colour");
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
        height: 10px;
        background-color: #bdbdbd;
        border-radius: 5px;
        border: 3px solid white; /* whatever the bg is */
        box-sizing: border-box;
    }

    :global(.bar-colour) {
        background-color: var(--primary) !important;
    }
</style>
