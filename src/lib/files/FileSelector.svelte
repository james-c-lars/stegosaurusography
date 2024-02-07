<script lang="ts">
    export let title = "This is a File";

    let selected_file: string | undefined = "C:/User/MickeyMouse/Downloads/feet.png";
    let status: string = "file-selected"

    function open(): void {
        selected_file = "C:/Users/MickeyMouse/Downloads/feet.png";
        status = "file-selected";
    }

    function close(): void {
        selected_file = undefined;
        status = "no-file-selected"
    }
</script>

<div class="file-select {status}">
    {#if selected_file}
        <img class="file-select-preview" src={selected_file} alt="The selected file" />
        <div class="file-select-top-info">
            <span class="file-select-title">{title}</span>
            <button class="file-select-close" on:click={close}>X</button>
        </div>
        <span class="file-select-path">{selected_file}</span>
    {:else}
        <button class="file-select-open" on:click={open}>+</button>
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

        padding: var(--interior-padding-y) var(--interior-padding-x);
    }

    .file-select.file-selected { 
        justify-content: space-between;
    }

    .file-select.no-file-selected {
        justify-content: center;
        align-items: center;
    }

    .file-select-preview {
        position: absolute;
        inset: 0;
    }

    .file-select-title, .file-select-close, .file-select-path, .file-select-open {
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
