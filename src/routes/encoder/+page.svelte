<script lang="ts">
    import FileSelector from "$lib/files/FileSelector.svelte";
    import { invoke } from "@tauri-apps/api";

    let baseFile: string | undefined = undefined;
    let secretFile: string | undefined = undefined;
    let encodedFile: string | undefined = undefined;

    async function encode() {
        await invoke("encode", {
            baseFile,
            secretFile,
            encodedFile,
        });
    }
</script>

<div class="encoder-view">
    <div class="encoder-options">
        <div class="encoder-option">
            <FileSelector bind:selected_file={baseFile} title="Base File" />
        </div>

        <div class="encoder-option">
            <div>
                <button>File</button>
                <button>Text</button>
            </div>
            <FileSelector bind:selected_file={secretFile} title="Secret File" />
        </div>
    </div>

    <div class="encoder-action">
        <button class="encode-button" on:click={encode} disabled={!(baseFile && secretFile)}>ENCODE!</button>
    </div>
</div>

<style>
    .encoder-view {
        height: 100%;
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        background: #2f2f2f;
    }

    .encoder-options {
        display: flex;
        flex-grow: 1;
    }

    .encoder-option {
        display: flex;
        flex-grow: 1;
        flex-direction: column;
    }

    .encoder-action {
        display: flex;
    }

    .encode-button {
        flex-grow: 1;
    }
</style>
