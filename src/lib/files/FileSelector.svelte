<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    export let title = "Base File";
    export let selected_file: string | undefined = undefined;

    async function openFile(): Promise<void> {
        const selection = await open({ multiple: false });

        selected_file = Array.isArray(selection) ? selection.at(0) : selection ?? undefined;
    }

    function closeFile(): void {
        selected_file = undefined;
    }
</script>

<div class="file-select" class:file-selected={selected_file}>
    {#if selected_file}
        <img class="file-select-preview" src={convertFileSrc(selected_file)} alt="The selected file" />
        <div class="file-select-top-info">
            <b class="file-select-title">{title}</b>
            <button class="file-select-close" on:click={closeFile}>✖</button>
        </div>
        <span class="file-select-path">{selected_file}</span>
    {:else}
        <button class="file-select-open" on:click={openFile}>+</button>
    {/if}
</div>

<style src="./styles.css">
    .file-select {
        width: 25ch;
        height: 25ch;
        position: relative;

        display: flex;
        flex-direction: column;

        border: 0.5ch solid aquamarine;
        border-radius: 5ch;
        overflow: hidden;

        align-items: center;
        justify-content: center;
    }

    .file-select.file-selected {
        justify-content: space-between;
        align-items: normal;
    }

    .file-select-preview {
        width: 100%;
        height: 100%;
        position: absolute;

        object-fit: cover;
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

    .file-select-title, .file-select-path {
        margin: 1ch 0;
        padding: 0 2ch;

        background-color: hsla(0, 0%, 0%, 0.3);
        color: white;
    }

    .file-select-close {
        padding: 0 1.5ch;
        margin: 0 1.5ch;

        background-color: transparent;
        border: none;

        color: red;
        font-size: 1em;
        -webkit-text-stroke: 1px hsla(0, 0%, 0%, 0.3);

        transform: scale(1.5);
    }
</style>
