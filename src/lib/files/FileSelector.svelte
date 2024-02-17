<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import "../../routes/styles.css";

    // Represents what kind of file is being selected
    export let title: string;
    // The path to the selected file. Undefined if no file is selected.
    export let selected_file: string | undefined = undefined;

    // This entire element is a large button. This click functions handles the interactions with it
    async function click(): Promise<void> {
        if (selected_file) {
            // If we click on a file select with an already selected file, clear it
            selected_file = undefined;
        } else {
            // Otherwise allow the user to pick a new file
            const selection = await open({ multiple: false });

            // open() has a few different return types. This is condensing them down into string | undefined
            selected_file = Array.isArray(selection) ? selection.at(0) : (selection ?? undefined);
        }
    }
</script>

<button class="file-select" class:file-selected={selected_file} on:click={click}>
    {#if selected_file}
        <!-- TODO: Figure out what to display if a non-image file is selected -->
        <!-- This image is in the background and shows the selected file -->
        <img class="file-select-preview" src={convertFileSrc(selected_file)} alt="The selected file" />

        <!-- This is the file title and the X to close the file, at the top of the <FileSelect> -->
        <div class="file-select-top-info">
            <span class="file-select-title">{title}</span>
            <span class="file-select-close">✖</span>
        </div>

        <!-- This is the path to the selected file at the bottom of the <FileSelect> -->
        <span class="file-select-path" title={selected_file}>{selected_file}</span>
    {:else}
        <!-- The name of what file will be selected by this element -->
        <span class="file-select-title">{title}</span>
        <!-- An icon representing that a file will be selected through interaction -->
        <span class="file-select-open">+</span>
    {/if}
</button>

<style>
    /* The root element */
    .file-select {
        width: 100%;
        height: 100%;
        position: relative;
        padding: 0;

        /* The image is in the background, so the flex will be for the top-info and the path */
        display: flex;
        flex-direction: column;
        align-items: normal;
        justify-content: space-between;

        /* The overall shape of the element */
        border-radius: 3rem;
        border: none;
        overflow: hidden;
        background: linear-gradient(180deg, var(--bg-high-contrast) 0%, var(--bg-low-contrast) 100%);
    }

    .file-select.file-selected {
        border: 0.3rem solid var(--accent-23);
    }

    /* The background image */
    .file-select-preview {
        width: 100%;
        height: 100%;
        position: absolute;

        object-fit: cover;
    }

    /* The parent of the title and X at the top of a <FileSelect> */
    .file-select-top-info {
        display: flex;
        justify-content: space-between;
    }

    .file-select-title, .file-select-path {
        position: relative; /* Needed so that it's in front of the aboslutely position background image */
        margin: 1rem 0;
        padding: 0.2rem 1rem;
    }

    /* The title that represents which type of file is being chosen */
    .file-select-title {
        font-size: 1.4em;
        font-weight: bold;
        color: var(--text-color);
    }

    /* Specifying that this is only when a file is selected, as title is also used
        for a <FileSelect> with no file */
    .file-select.file-selected .file-select-title,
    .file-select.file-selected .file-select-path {
        background-color: hsla(0, 0%, 0%, 0.3);
        color: white;
    }

    /* The file path at the bottom of the <FileSelect> */
    .file-select-path {
        overflow: hidden;

        white-space: nowrap;
        text-overflow: ellipsis;
    }

    /* The X at the top of a <FileSelect> when a file is selected */
    .file-select-close {
        height: fit-content;
        position: relative;
        margin: auto 1rem auto 0;

        color: var(--cancel-color);
        font-size: 3em;
        line-height: 1em;

        transition: font-size 0.1s; /* Used for hover effect */
    }

    .file-select:hover .file-select-close {
        font-size: 3.3em;
    }

    /* The + in an empty <FileSelect> that represents a file can be chosen */
    .file-select-open {
        position: absolute;
        inset: 0;

        display: flex;
        align-items: center;
        justify-content: center;

        color: var(--accept-color);
        font-size: 5em;

        transition: font-size 0.1s; /* Used for hover effect */
    }

    .file-select:hover .file-select-open {
        font-size: 6.2em;
    }
</style>
