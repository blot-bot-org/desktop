<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";

  let num_lines = $state(1);

  async function gen_preview(event: Event) {
      event.preventDefault();

      let path = await invoke("gen_preview", { jsonParams: `{"num_lines":${num_lines}}` });
      let imageUrl = convertFileSrc(path);

      imageUrl += `?refresh=${Math.floor(Math.random() * 10000000)}`; // hack to force reload image by adding random parameter to change url

      let imageElement = document.getElementById("preview-image").src = imageUrl;

  }
</script>

<main class="container">
    <button on:click={gen_preview}>Go</button>
    <input type="range" min="1" max="32" bind:value={num_lines} />
    <img id="preview-image" loading="lazy" />

</main>

<style>
</style>
