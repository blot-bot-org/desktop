<script>
    export let previewRef;
    
    (async () => {
        let i = 0;
        while (previewRef == undefined) {
            await new Promise(r => setTimeout(r, 2000));
            i += 1;

            if (i > 1000) { // 10 seconds
                console.log("Error! `previewRef` was never intialised in dashboard");
                break;
            }
        }

        document.getElementById("preview-image").src = previewRef.getImageUrl();
        console.log(`set image url to ${previewRef.getImageUrl()}`)
    })();
</script>

<div id="fixed-background" />
<div id="client-window">
    <div class="content">
        
        <!-- flex box containing stats and preview -->
        <div class="stat-preview">
            <div class="stat-container"></div>
            <div class="preview-container">
                <a class="currently-drawing">Currently drawing</a>
                <img id="preview-image" />
            </div>
        </div>

        <!-- progress bar and buttons -->
        <div class="buttons-progress">
        </div>

    </div>
</div>

<style>
    #fixed-background {
        position: fixed;
        width: 100%;
        height: 100%;

        background-color: #000000c5;
    }

    #client-window {
        position: fixed;

        background-color: #FFFFFF;
        width: 800px;
        height: 600px;
        border-radius: 5px;

        left: calc(50% - 400px);
        top: calc(50% - 300px);

        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);
    }

    #preview-image {
        margin: 20px;
        width: 90%;

        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);
    }
    

    .content {
        padding: 30px;
        box-sizing: border-box;

        width: 100%;
        height: 100%;
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

        background-color: yellow;
    }

    .preview-container {
        width: 40%;
    }

    .currently-drawing {
        width: 100%;

        text-align: center;
        text-transform: uppercase;

        color: #a0a0a0;
        font-size: 12px;
        font-weight: 600;
        letter-spacing: 0.1px;
    }

</style>
