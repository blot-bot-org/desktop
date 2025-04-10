<script lang="ts">
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  
    let imageSrc = $state("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAANIAAAEpAQMAAADcde5vAAAAAXNSR0IB2cksfwAAAAlwSFlzAAALEwAACxMBAJqcGAAAAANQTFRF////p8QbyAAAAB9JREFUGBntwTEBAAAAwiD7p14IX2AAAAAAAAAAAIcAIHwAAU/BTAIAAAAASUVORK5CYII=");
  
    export async function gen_preview(parameter_object) {
        console.log(`Invoking preview function with obj:`);
        console.log(parameter_object);
        let path = await invoke("gen_preview", { jsonParams: JSON.stringify(parameter_object) });
        let imageUrl = convertFileSrc(path);
  
        imageUrl += `?refresh=${Math.floor(Math.random() * 10000000)}`; // hack to force reload image by adding random parameter to change url
        imageSrc = imageUrl;
  
    }
  
    export function scaleImage(event: Event) {
        let imageWidth = event.srcElement.naturalWidth;
        let imageHeight = event.srcElement.naturalHeight;
  
        if(imageWidth > imageHeight) {
            event.srcElement.style.width = "80%";
            event.srcElement.style.height = "auto";
        } else {
            event.srcElement.style.height = "80%";
            event.srcElement.style.width = "auto";
        }

        // error if the window gets too thin but whatever
    }
  
     
</script>

<div id="preview-container">
    <div id="preview-image-container">
        <img id="preview-image" src={imageSrc} on:load={scaleImage} />
    </div>
</div>

<style>
    #preview-container {
        background-color: #D0D0D0;
        height: 100%;
        width: 65%; /* master width */

        box-shadow: inset -68px 0px 18px -67px rgba(0,0,0,0.08);

        box-sizing: border-box;
        border-right: 1px solid #C0C0C0;
    }

    #preview-image-container {
        height: 100%;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    #preview-image {
        max-width: 80%;
        max-height: 80%;
        object-fit: contain;
         
        box-shadow: 0px 13px 36px -5px rgba(0,0,0,0.15),0px 26px 50px 24px rgba(0,0,0,0.07),0px 8px 18px -1px rgba(0,0,0,0.2);
    }
</style>
