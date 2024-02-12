<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import "../../routes/styles.css";

    export let title = "Base File";
    export let selected_file: string | undefined = undefined;

    async function click(): Promise<void> {
        if (selected_file) {
            selected_file = undefined;
        } else {
            const selection = await open({ multiple: false });

            selected_file = Array.isArray(selection) ? selection.at(0) : (selection ?? undefined);
        }
    }
</script>

<button class="file-select" class:file-selected={selected_file} on:click={click}>
    {#if selected_file}
        <img class="file-select-preview" src={convertFileSrc(selected_file)} alt="The selected file" />
        <div class="file-select-top-info">
            <span class="file-select-title">{title}</span>
            <span class="file-select-close">✖</span>
        </div>
        <span class="file-select-path" title={selected_file}>{selected_file}</span>
    {:else}
        <span class="file-select-title">{title}</span>
        <span class="file-select-open">+</span>
    {/if}
</button>

<style>
    .file-select {
        width: 100%;
        height: 100%;
        position: relative;
        padding: 0;

        display: flex;
        flex-direction: column;
        align-items: normal;
        justify-content: space-between;

        border-radius: 3rem;
        border: none;
        overflow: hidden;
        background: linear-gradient(180deg, hsl(0, 0%, 12%) 0%, hsl(0, 0%, 16%) 100%);
    }

    .file-select.file-selected {
        border: 0.3rem solid hsl(290, 99%, 23%);
    }

    .file-select-preview {
        width: 100%;
        height: 100%;
        position: absolute;

        object-fit: cover;
    }

    .file-select-top-info {
        display: flex;
        justify-content: space-between;
    }

    .file-select-title, .file-select-path {
        position: relative;
        margin: 1rem 0;
        padding: 0.2rem 1rem;

        color: white;
    }

    .file-select-title {
        font-size: 1.4em;
        font-weight: bold;
    }

    .file-select.file-selected .file-select-title {
        background-color: hsla(0, 0%, 0%, 0.3);
    }

    .file-select-path {
        overflow: hidden;
        
        background-color: hsla(0, 0%, 0%, 0.3);

        white-space: nowrap;
        text-overflow: ellipsis;
    }

    .file-select-close {
        height: fit-content;
        position: relative;
        margin: auto 1rem auto 0;

        color: darkred;
        font-size: 3em;
        line-height: 1em;

        transition: font-size 0.1s;
    }

    .file-select:hover .file-select-close,
    .file-select:focus .file-select-close {
        font-size: 3.3em;
    }

    .file-select-open {
        position: absolute;
        inset: 0;

        display: flex;
        align-items: center;
        justify-content: center;

        color: darkgreen;
        font-size: 5em;

        transition: font-size 0.1s;
    }

    .file-select:hover .file-select-open,
    .file-select:focus .file-select-open {
        font-size: 6.2em;
    }
</style>
