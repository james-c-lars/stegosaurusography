<script lang="ts">
    import { save } from "@tauri-apps/plugin-dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined;
    export let dialogTitle = "Save File";
    export let inputFileExtensions: string[];

    function selectFunction(): Promise<string | undefined> {
        return save({
            title: dialogTitle,
            filters: [
                {
                    name: "Base File Type",
                    extensions: inputFileExtensions,
                },
            ],
        }).then((file) => file ?? undefined);
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
