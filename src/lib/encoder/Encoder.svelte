<script lang="ts">
    import FileOpenInput from "../shared/FileOpenInput.svelte";
    import FileSaveInput from "../shared/FileSaveInput.svelte";
    import { invoke } from "@tauri-apps/api";

    let baseFile: string | undefined = undefined;
    let secretFile: string | undefined = undefined;
    let encodedFile: string | undefined = undefined;

    async function encode() {
        await invoke("encode", {
            baseFile,
            secretFile,
            outputFile: encodedFile,
        });
    }

    function fileExtension(fileName: string): string {
        return fileName.split(".").pop() ?? "";
    }
</script>

<div class="encoder-view">
    <FileOpenInput bind:selectedFile={baseFile} dialogTitle="Select Base File" />
    <p>Base File: {baseFile}</p>

    <FileOpenInput bind:selectedFile={secretFile} dialogTitle="Select Secret File" />
    <p>Secret File: {secretFile}</p>

    {#if baseFile}
        <FileSaveInput
            bind:selectedFile={encodedFile}
            dialogTitle="Save Encoded File"
            inputFileExtensions={[fileExtension(baseFile)]}
        />
        <p>Encoded File: {encodedFile}</p>
    {/if}

    {#if baseFile && secretFile && encodedFile}
        <button on:click={encode}>ENCODE!</button>
    {/if}
</div>

<style>
    .encoder-view {
        background: #2f2f2f;
    }
</style>
