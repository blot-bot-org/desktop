<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    const props: {
        onDraw(): void,
        onError(): void
    } = $props();

    let progress: number = $state(0);
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
            ModalLayout.applyLayout(new ModalLayout(false, "", "The pen is moving towards the starting position. Please wait..."));
            progress = 1;

            await invoke("move_pen_to_start")
                .then(() => {
                    ModalLayout.applyLayout(new ModalLayout(true, "Start", "Drawing is ready. Press 'Start' to begin a 5 second countdown."));
                    progress = 2;
                })
                .catch((err) => {
                    ModalLayout.applyLayout(new ModalLayout(true, "Close", err));
                    progress = -1;
                });
        } else if(progress == 2) {
            ModalLayout.applyLayout(new ModalLayout(false, "", "Drawing is starting..."));

            await new Promise(r => setTimeout(r, 5000));
            
            props.onDraw();
        }
    }
</script>


<div id="modal-container">

    <div id="progress-container">

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
    #modal-container {
        width: 100% !important;
        height: 100% !important;

        display: flex;
        flex-direction: column;
    }

    #progress-container {
        width: 100%;
        height: 50px;

        background-color: #f0f0f0;
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
        background-color: #3384F0;
        border: 1px solid #2374F0;
        padding: 5px 10px 6px 10px;

        border-radius: 5px;

        color: white;
        box-shadow: 0px 5px 12px -1px #00000030;

        cursor: pointer;
        transition: 0.25s background-color;
    }

    .button:hover {
        background-color: #2374F0;
    }
</style>
