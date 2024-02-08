<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";

    export let title = "This is a File";

    let selected_file: string | undefined = undefined;

    async function openFile(): Promise<void> {
        const selection = await open();

        selected_file = Array.isArray(selection) ? selection.at(0) : selection ?? undefined;
    }

    function closeFile(): void {
        selected_file = undefined;
    }
</script>

<div class="file-select" class:file-selected={selected_file}>
    {#if selected_file}
        <img class="file-select-preview" src={selected_file} alt="The selected file" />
        <div class="file-select-top-info">
            <span class="file-select-title">{title}</span>
            <button class="file-select-close" on:click={closeFile}>X</button>
        </div>
        <span class="file-select-path">{selected_file}</span>
    {:else}
        <button class="file-select-open" on:click={openFile}>+</button>
    {/if}
</div>

<style>
    .file-select {
        --total-width: 25ch;
        --total-height: var(--total-width);
        --interior-padding-x: 2ch;
        --interior-padding-y: var(--interior-padding-x);

        width: calc(var(--total-width) - 2 * var(--interior-padding-x));
        height: calc(var(--total-height) - 2 * var(--interior-padding-y));
        position: relative;

        display: flex;
        flex-direction: column;

        border: 0.5ch solid aquamarine;
        border-radius: 5ch;
        overflow: hidden;

        align-items: center;
        justify-content: center;

        padding: var(--interior-padding-y) var(--interior-padding-x);
    }

    .file-select.file-selected {
        justify-content: space-between;
        align-items: normal;
    }

    .file-select-preview {
        position: absolute;
        inset: 0;
    }

    .file-select-title,
    .file-select-close,
    .file-select-path,
    .file-select-open {
        position: relative;
    }

    .file-select-top-info {
        display: flex;
        justify-content: space-between;
    }

    .file-select-path {
        overflow: hidden;

        white-space: nowrap;
        text-overflow: ellipsis;
        font-size: 0.7em;
    }
</style>
