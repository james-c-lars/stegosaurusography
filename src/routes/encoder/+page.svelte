<script lang="ts">
    import FileSelector from "$lib/files/FileSelector.svelte";
    import { invoke } from "@tauri-apps/api";
    import "../styles.css";

    let textBasedSecret: boolean = false;

    let baseFile: string | undefined = undefined;
    let secretFile: string | undefined = undefined;
    let outputFile: string | undefined = undefined;

    async function encode() {
        await invoke("encode", {
            baseFile,
            secretFile,
            outputFile,
        });
    }
</script>

<div class="encoder-view">
    <FileSelector bind:selected_file={baseFile} title="Base File" />

    <span>
        <button on:click={() => textBasedSecret = false}>File</button>
        <button on:click={() => textBasedSecret = true}>Text</button>
    </span>
    {#if textBasedSecret}
        <textarea />
    {:else}
        <FileSelector bind:selected_file={secretFile} title="Secret File" />
    {/if}

    <button class="encode-button" disabled={!(baseFile && secretFile)} on:click={encode} 
            title={!(baseFile && secretFile) ? "Select a base file and a secret to encode into it" : ""}>
        Encode
    </button>
</div>

<style src="../styles.css">
    .encoder-view {
        width: 100%;
        height: 100%;
        padding: 1rem 1.5rem;

        display: grid;
        gap: 1rem 1.5rem;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr auto;

        background: hsl(0, 0%, 14%);
    }

    :global(.encoder-view > *:first-child) {
        grid-row: span 2;
    }

    .encode-button {
        width: 100%;
        height: 3rem;
        max-width: 45rem;
        margin: 0 auto;

        grid-column: span 2;

        border: none;
        background: linear-gradient(90deg, hsl(290, 99%, 30%) 0%, hsl(290, 99%, 33%) 100%);
        
        color: white;
        font-weight: bolder;
        font-size: 1.5em;
        transition: font-size .1s;
    }

    .encode-button:disabled {
        background: linear-gradient(90deg, hsl(290, 99%, 12%) 0%, hsl(290, 99%, 13%) 100%);
        color: grey;
    }

    .encode-button:enabled:hover,
    .encode-button:enabled:focus {
        background: linear-gradient(90deg, hsl(290, 99%, 27%) 0%, hsl(290, 99%, 30%) 100%);
        font-size: 1.6em;
    }
</style>
