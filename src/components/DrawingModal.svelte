<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import StepBar from "$components/steps/StepBar.svelte";

    const steps = [
        { text: "" },
        { text: "" },
        { text: "" },
        { text: "" },
    ];

    const props: {
        onDraw(): void,
        onError(): void
    } = $props();

    let stepValue = $state(0);
    let progress: number = $state(0);
    let isError = $derived((() => progress == -1)());

    let buttonText = $state("Done");
    let buttonShowing = $state(true);
    let text = $state("");
    
    class ModalLayout {
        showButton: bool;
        buttonText: string;
        text: string;

        constructor(showButton: bool, buttonText: string, text: string) {
            this.showButton = showButton;
            this.buttonText = buttonText;
            this.text = text;
        }

        public static applyLayout(self: ModalLayout) {
            buttonShowing = self.showButton;
            buttonText = self.buttonText;

            if(self.text != undefined) {
                text = self.text;
            }
        }
    }

    ModalLayout.applyLayout(new ModalLayout(true, "Done", "Place the pen at the top left of the page. Ensure the pen lid is on."));

    // 0 -> move pen to 0, 0
    // 1 -> pen moving to new position
    // 2 -> start drawing in x seconds
    // -1 -> error

    async function buttonPressed(): void {

        if(progress == -1) {
            props.onError();
        }

        if(progress == 0) {
            ModalLayout.applyLayout(new ModalLayout(true, "Cancel", "The pen is moving towards the starting position. Please wait..."));
            progress = 1;
            stepValue = 1;

            await invoke("move_pen_to_start")
                .then(() => {
                    ModalLayout.applyLayout(new ModalLayout(true, "Start", "Drawing is ready to begin. Press 'Start' to begin a 3 second countdown."));
                    progress = 2;
                    stepValue = 2;
                })
                .catch((err) => {
                    ModalLayout.applyLayout(new ModalLayout(true, "Close", "Error: " + err));
                    progress = -1;
                });
        } else if (progress == 1) {
            props.onError();
        } else if(progress == 2) {
            stepValue = 3;
            ModalLayout.applyLayout(new ModalLayout(false, "", "Drawing is starting..."));

            await new Promise(r => setTimeout(r, 3000)); // gives time to run over and move pen etc
            
            props.onDraw();
        }
    }
</script>


<div id="modal-container">

    <div id="progress-container">
        <StepBar activeWidth={40} numBars={4} bind:progress={stepValue} bind:isError={isError} />
    </div>

    
    <div id="modal-content">
        <div class="instruction-text">{text}</div>
    </div>


    <div id="modal-controls">
        {#if buttonShowing}
            <button class="button" onclick={() => { buttonPressed(); } }>{buttonText}</button>
        {/if}
    </div>

</div>


<style>
    :global(.steps__number) {
        color: transparent;
        user-select: none;
    }

    :global(.steps-container) {
    }

    #modal-container {
        width: 100% !important;
        height: 100% !important;

        display: flex;
        flex-direction: column;
    }

    #progress-container {
        display: flex;

        width: 100%;
        height: 50px;

        align-items: center;
        justify-content: center;
    }

    #modal-content {
        flex-grow: 1;

        display: flex;
        justify-content: center;
        align-items: center;
    }

    #modal-controls {
        display: flex;
        justify-content: flex-end;

        height: 30px;
    }


    .instruction-text {
        width: 50%;
        text-align: center;
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

    .button:hover {
        background-color: var(--primary-selected);
    }
</style>
