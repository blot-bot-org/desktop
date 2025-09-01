<script>
    import { onMount } from "svelte";

    import 'tippy.js/dist/tippy.css';
    import 'tippy.js/animations/scale.css';
    import tippy from "tippy.js";

    export let element;
    export let text;

    // parameters for the popups
    onMount(() => {
        tippy(element, {
            content: text,
            delay: [100, 0],
            duration: [200, 200],
            placement: "left",
            animation: "fade",
            theme: 'light',
        });
    });
</script>
