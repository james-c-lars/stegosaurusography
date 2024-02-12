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

    function change(event: Event) {
        textBasedSecret = (event.target as HTMLInputElement | null)?.value === "text";
    }
</script>

<div class="encoder-view">
    <FileSelector bind:selected_file={baseFile} title="Base File" />

    <span class="text-or-file-select">
        <label class="text-or-file" class:checked={!textBasedSecret}>
            File
            <input type="radio" name="text-or-file" value="file" on:change={change} checked />
        </label>
        <label class="text-or-file" class:checked={textBasedSecret}>
            Text
            <input type="radio" name="text-or-file" value="text" on:change={change} />
        </label>
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
        padding: 1rem 1.5rem 2rem 1.5rem;

        display: grid;
        gap: 1rem 1.5rem;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr auto;
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

    input[name = "text-or-file"] {
        appearance: none;

        position: absolute;
        inset: 0;
        margin: 0.125rem;
    }

    .text-or-file-select {
        display: inline-flex;
    }

    .text-or-file {
        --border-radius: 15%;

        position: relative;
        width: 3rem;
        height: 2rem;

        border: 0.125rem solid hsl(290, 99%, 30%);

        color: grey;
        text-align: center;
        line-height: 1.75rem;

        transition: font-size 0.1s;
    }

    .text-or-file:first-child {        
        border-radius: var(--border-radius) 0 0 var(--border-radius);
        border-right: 0;
    }

    .text-or-file:last-child {
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        border-left: 0;
    }

    .text-or-file.checked {
        background: linear-gradient(90deg, hsl(290, 99%, 30%) 0%, hsl(290, 99%, 31%) 100%);
        color: white;
    }

    .text-or-file:hover {
        color: white;
        font-size: 1.0625em;
    }
</style>
