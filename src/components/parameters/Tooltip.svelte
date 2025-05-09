<script>
    import { onMount } from "svelte";

    import 'tippy.js/dist/tippy.css';
    import 'tippy.js/animations/scale.css';
    import tippy from "tippy.js";

    export let element;
    export let text;

    onMount(() => {
        tippy(element, {
            content: text,
            delay: [1000, 0],
            duration: [300, 300],
            placement: "left",
            animation: "fade",
            theme: 'light',
        });
    });
</script>
