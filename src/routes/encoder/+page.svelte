<script lang="ts">
    import FileSelector from "$lib/files/FileSelector.svelte";
    import { invoke } from "@tauri-apps/api";
    import "../styles.css";

    // Whether we are using user-entered text as the secret to encode into a file
    // The alternative is encoding a whole file in the base file
    let textBasedSecret: boolean = false;

    // The file path into which a secret will be encoded
    let baseFile: string | undefined = undefined;
    // The file path that's a secret file that will be encoded into the base file
    let secretFile: string | undefined = undefined;
    // A file path that the encoded output file will be saved to
    let outputFile: string | undefined = undefined;

    // Once all the required information has been collected, this function is used
    // to call into our encoding functionality in Rust
    async function encode() {
        // TODO: Need to get outputFile before invoking encode

        await invoke("encode", {
            baseFile,
            secretFile,
            outputFile,
        });
    }

    // Called whenever the radio buttons for File and Text change value
    function change(event: Event) {
        // The value of the buttons are "text" and "file"
        textBasedSecret = (event.target as HTMLInputElement | null)?.value === "text";
    }
</script>

<div class="encoder-view">
    <!-- The Base File selector -->
    <FileSelector bind:selected_file={baseFile} title="Base File" />

    <!-- The radio buttons used to toggle between a secret file, and user-entered text -->
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
        <!-- TODO: Need to style this textarea -->
        <!-- Where a user can enter text to directly be encoded -->
        <textarea />
    {:else}
        <!-- The secret file selector -->
        <FileSelector bind:selected_file={secretFile} title="Secret File" />
    {/if}

    <!-- The button that finalizes user input and calls our encoding logic -->
    <button class="encode-button" disabled={!(baseFile && secretFile)} on:click={encode} 
            title={!(baseFile && secretFile) ? "Select a base file and a secret to encode into it" : ""}>
        Encode
    </button>
</div>

<style>
    /* The root of the element */
    .encoder-view {
        width: 100%;
        height: 100%;
        padding: 1rem 1.5rem 2rem 1.5rem;

        /* Using grid to get a consistent layout at many different screen sizes */
        display: grid;
        gap: 1rem 1.5rem;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: auto 1fr auto;
    }

    /* This will target the Base File selector, to make it vertically larger */
    :global(.encoder-view > *:first-child) {
        grid-row: span 2;
    }

    /* The button used to encode the secret into the base file */
    .encode-button {
        width: 100%;
        height: 3rem;
        max-width: 45rem; /* Capping the encode buttons width */
        margin: 0 auto; /* Centering the encode button */

        /* Making the encode button take the entire space at the bottom */
        grid-column: span 2;

        border: none; /* Buttons by default have a border */
        background: linear-gradient(90deg, var(--accent-30) 0%, var(--accent-33) 100%);
        
        color: var(--text-on-accent);
        font-weight: bolder;
        font-size: 1.5em;
        transition: font-size .1s; /* Font-size changes via hover effect */
    }

    .encode-button:disabled {
        background: linear-gradient(90deg, var(--accent-12) 0%, var(--accent-13) 100%);
        color: var(--text-deemphasized);
    }

    .encode-button:enabled:hover,
    .encode-button:enabled:focus {
        background: linear-gradient(90deg, var(--accent-27) 0%, var(--accent-30) 100%);
        font-size: 1.6em;
    }

    /* The actual input elements underlying the radio buttons are invisible.
        We make them the same size and shape as the labels that are actually
        used to interact with the input elements, so that focus looks right.
        Labels can't be focused, only input elements can be. */
    input[name = "text-or-file"] {
        appearance: none;

        position: absolute;
        inset: 0; /* Making the input element the same size as its parent label */
        margin: 0.125rem;

        background-color: transparent;
    }

    /* The parent of the text / file radio buttons */
    .text-or-file-select {
        /* Removes spacing between our two radio buttons */
        display: inline-flex;
    }

    /* Both the text and file radio buttons */
    .text-or-file {
        --border-radius: 0.5rem;

        position: relative;
        width: 3rem;
        height: 2rem;

        border: 0.125rem solid var(--accent-32);

        color: var(--text-deemphasized);
        text-align: center;
        line-height: 1.75rem; /* A magic number for centering the text in the buttons */
        /* TODO: Find a better way to center this button */

        transition: font-size 0.1s; /* Font-size changes via hover effect */
    }

    .text-or-file:first-child {   
        /* Curving the left side of the leftmost button */     
        border-radius: var(--border-radius) 0 0 var(--border-radius);
        border-right: 0;
    }

    .text-or-file:last-child {
        /* Curving the right side of the rightmost button */
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        border-left: 0;
    }

    /* Increasing text contrast and size on hover */
    .text-or-file:hover {
        color: var(--text-color);
        font-size: 1.0625em;
    }

    /* The checked option will swap fill and text color */
    .text-or-file.checked {
        background: linear-gradient(90deg, var(--accent-30) 0%, var(--accent-33) 100%);
        color: var(--text-on-accent);
    }
</style>
