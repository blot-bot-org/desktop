<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    import DrawingModal from "./DrawingModal.svelte";

    const props: {
        previewRef: any;
        close(): void;
    } = $props();

    let isDrawing = $state(false); // false
    let isIntro = $state(true); // true


    let msState = $state("...");
    let msAddress = $state("");
    let drInstructionIdx = $state("");
    let drTotalInstructions = $state("");
    let drTimeElapsed = $state("");
    let drTimeRemaining = $state("");
    let mcMaxInsBytes = $state("");
    let mcMaxStepSpeed = $state("");
    let mcPulseWidth = $state("");
    let mcProtocol = $state("");


    let drawingPaused = $state(false);

    let secondsElapsed = 0;
    let secondsRemaining = 0;
    let bySecondUpdateInterval = undefined;



    async function startDrawing() {
        secondsElapsed = 0;
        secondsRemaining = 0;
        if(bySecondUpdateInterval != undefined) {
            clearInterval(bySecondUpdateInterval);
        }

        // initialise bits and bobs
        // document.getElementById("modal-preview-img").src = props.previewRef.getImageUrl();
        const emptyParams: UpdateWindowParams = {
            _msState: "Connecting",
            _msAddress: "...",
            _drInstructionIdx: "...",
            _drTotalInstructions: "...",
            _drTimeElapsed: "00:00:00",
            _drTimeRemaining: "00:00:00",
            _mcMaxInsBytes: "...",
            _mcMaxStepSpeed: "...",
            _mcPulseWidth: "...",
            _mcProtocol: "..."
        };
        updateWindow(emptyParams);



        
        // start progress listener
        let firmware_progress = await listen<string>("firm-prog", (ev) => { try { handleProgress(JSON.parse(ev.payload)); } catch { console.error(ev); } });
        await invoke("send_to_firmware"); // call drawing function

        firmware_progress(); // cancels listener
    }

    function handleProgress(payload) {
        console.log(payload)

        if(payload["event"] == "populate_network") {
            updateWindow({ _msAddress: payload["address"] });
        }

        if(payload["event"] == "populate_draw") {
            updateWindow({ _drTotalInstructions: payload["totalBytes"] });
        }

        if(payload["event"] == "populate_machine") {
            updateWindow({ 
                _mcMaxInsBytes: payload["insBytes"],
                _mcMaxStepSpeed: payload["stepSpeed"],
                _mcPulseWidth: payload["pulseWidth"],
                _mcProtocol: payload["protocol"],
            });
        }

        if(payload["event"] == "connection") {
            updateWindow({ _msState: "Drawing" });

            bySecondUpdateInterval = setInterval(bySecondUpdate, 1000);
        }

        if(payload["event"] == "drawing") {

            secondsRemaining = parseInt(payload["secs_remaining"]);
            updateWindow({ _drInstructionIdx: payload["ins_pos"] });

        }
    }

    function bySecondUpdate() {
        let hours = String(Math.floor(secondsElapsed / 3600)).padStart(2, '0');
        let minutes = String(Math.floor((secondsElapsed % 3600) / 60)).padStart(2, '0');
        let seconds = String(Math.floor(secondsElapsed % 60)).padStart(2, '0');
        let remainingHours = String(Math.floor(secondsRemaining / 3600)).padStart(2, '0');
        let remainingMinutes = String(Math.floor((secondsRemaining % 3600) / 60)).padStart(2, '0');
        let remainingSeconds = String(Math.floor(secondsRemaining % 60)).padStart(2, '0');

        updateWindow({ _drTimeElapsed: `${hours}:${minutes}:${seconds}`, _drTimeRemaining: `${remainingHours}:${remainingMinutes}:${remainingSeconds}` });

        secondsElapsed += 1;
        secondsRemaining = Math.max(secondsRemaining - 1, 0);
    }

    type UpdateWindowParams = {
        _msState?: string;
        _msAddress?: string;
        _drInstructionIdx?: string;
        _drTotalInstructions?: string;
        _drTimeElapsed?: string;
        _drTimeRemaining?: string;
        _mcMaxInsBytes?: string;
        _mcMaxStepSpeed?: string;
        _mcPulseWidth?: string;
        _mcProtocol?: string;
    };

    export function updateWindow(params: UpdateWindowParams = {}) {
        const {
            _msState,
            _msAddress,
            _drInstructionIdx,
            _drTotalInstructions,
            _drTimeElapsed,
            _drTimeRemaining,
            _mcMaxInsBytes,
            _mcMaxStepSpeed,
            _mcPulseWidth,
            _mcProtocol,
        } = params;

        if(_msState != undefined) { msState = _msState; };
        if(_msAddress != undefined) { msAddress = _msAddress };
        if(_drInstructionIdx != undefined) { drInstructionIdx = _drInstructionIdx };
        if(_drTotalInstructions != undefined) { drTotalInstructions = _drTotalInstructions };
        if(_drTimeElapsed != undefined) { drTimeElapsed = _drTimeElapsed };
        if(_drTimeRemaining != undefined) { drTimeRemaining = _drTimeRemaining };
        if(_mcMaxInsBytes != undefined) { mcMaxInsBytes = _mcMaxInsBytes };
        if(_mcMaxStepSpeed != undefined) { mcMaxStepSpeed = _mcMaxStepSpeed };
        if(_mcPulseWidth != undefined) { mcPulseWidth = _mcPulseWidth };
        if(_mcProtocol != undefined) { mcProtocol = _mcProtocol };
    }

    setTimeout(() => {
        // startDrawing();
    }, 100);
</script>

{#if isDrawing || isIntro}
    <div id="fixed-background" />
{/if}

{#if isDrawing || isIntro}
<div id="client-window">
    <div class="content">

        
        {#if isDrawing}
        <!-- flex box containing stats and preview -->
        <div class="stat-preview">
            <div class="stat-container">
                
                <a class="stat-section">Connection Information</a> 
                <div class="stat-box">
                    <div class="stat-entry">
                        <a class="stat-title">Machine State</a>
                        <a class="stat-value" id="msState">{msState}</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Machine Address</a>
                        <a class="stat-value" id="msAddress">{msAddress}</a>
                    </div>
                </div>

                <a class="stat-section">Drawing Information</a> 
                <div class="stat-box">
                    <div class="stat-entry">
                        <a class="stat-title">Instruction Index</a>
                        <a class="stat-value">{drInstructionIdx}</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Total Instructions</a>
                        <a class="stat-value">{drTotalInstructions} bytes</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Time Elapsed</a>
                        <a class="stat-value">{drTimeElapsed}</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Time Remaining</a>
                        <a class="stat-value">~{drTimeRemaining}</a>
                    </div>
                </div>

                <a class="stat-section">Machine Configuration</a> 
                <div class="stat-box">
                    <div class="stat-entry">
                        <a class="stat-title">Instruction Buffer Size</a>
                        <a class="stat-value">{mcMaxInsBytes} bytes</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Max Step Speed</a>
                        <a class="stat-value">{mcMaxStepSpeed} steps/sec</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Pulse Width</a>
                        <a class="stat-value">{mcPulseWidth}μs</a>
                    </div>
                    <div class="stat-entry">
                        <a class="stat-title">Protocol</a>
                        <a class="stat-value">v {mcProtocol}</a>
                    </div>
                </div>

            </div>
            <div class="preview-container">
                <a class="currently-drawing">Currently drawing</a>
                <img id="modal-preview-img" />
            </div>
        </div>

        <!-- progress bar and buttons -->
        <div class="buttons-progress">
        </div>
        {:else if isIntro}
            <DrawingModal 
                onDraw={() => { console.log("drawing"); isIntro = false; isDrawing = true; startDrawing(); }}
                onError={() => { console.log("EREROR!"); isIntro = false; props.close(); }}
            />
        {/if}

    </div>
</div>
{/if}

<style>
    :global(#fixed-background) {
        position: fixed;
        width: 100% !important;
        height: 100% !important;

        background-color: #000000c5;
    }

    #client-window {
        position: fixed;

        background-color: #FFFFFF;
        width: 800px !important;
        height: 600px !important;
        display: block;
        border-radius: 5px;

        left: calc(50% - 400px);
        top: calc(50% - 300px);

        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);
    }

    #modal-preview-img {
        margin: 20px;

        width: calc(100% - 40px);

        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.10),0px 26px 50px 24px rgba(0,0,0,0.03),0px 8px 18px -1px rgba(0,0,0,0.1);
        object-fit: contain;
    }


    .content {
        padding: 30px;
        box-sizing: border-box;

        width: 100% !important;
        height: 100% !important;

        display: block;
    }

    .stat-preview {
        display: flex;

        width: 100%;
        height: 88%;
    }
    
    .buttons-progress {
        background-color: green;

        width: 100%;
        height: 12%;
    }

    .stat-container {
        width: 60%;

        display: block;
    }

    .preview-container {
        width: 40%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .currently-drawing {
        width: 100%;

        text-align: center;
        text-transform: uppercase;

        color: #a0a0a0;
        font-size: 13px;
        font-weight: 600;
        letter-spacing: 0.1px;
    }

    .stat-box {
        width: calc(100% - 40px);

        margin: 20px;
        padding-bottom: 30px;
        padding-top: 10px;

        display: flex;
        flex-wrap: wrap;
        gap: 10px; /* THIS HERE */
        justify-content: space-between;

        border-bottom: 1px solid #D0D0D0;

    }

    .stat-entry {
        display: flex;
        flex-direction: column;

        flex: 0 0 calc(50% - 10px); /* REFERS TO THIS HERE */
    }

    .stat-title {
        text-transform: uppercase;
        font-size: 13px;
        font-weight: 600;
        letter-spacing: 0.1px;
    }

    .stat-section {
        width: 100% !important;
        text-align: center;

        display: block;

        text-transform: uppercase;
        color: #a0a0a0;
        font-size: 13px;
        font-weight: 600;
        letter-spacing: 0.1px;
    }

</style>
