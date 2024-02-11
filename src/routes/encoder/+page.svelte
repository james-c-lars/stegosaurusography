<script lang="ts">
    import FileSelector from "$lib/files/FileSelector.svelte";
    import { invoke } from "@tauri-apps/api";

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
        <input />
    {:else}
        <FileSelector bind:selected_file={secretFile} title="Secret File" />
    {/if}
    <button class="encode-button" on:click={encode} disabled={!(baseFile && secretFile)}>ENCODE!</button>
</div>

<style src="./styles.css">
    .encoder-view {
        width: 100%;
        height: 100%;
        padding: 0.5rem 1.5rem;

        display: grid;
        gap: 0.5rem 1.5rem;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr auto;

        background: #2f2f2f;
    }

    :global(.encoder-view > *:first-child) {
        grid-row: span 2;
    }

    .encode-button {
        width: 100%;
        height: 3rem;

        grid-column: span 2;
    }
</style>
