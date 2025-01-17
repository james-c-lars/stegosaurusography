<script lang="ts">
    import { save } from "@tauri-apps/plugin-dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined;
    export let dialogTitle = "Save File";
    export let inputFileExtensions: string[];

    async function selectFunction(): Promise<string | undefined> {
        const file = await save({
            title: dialogTitle,
            filters: [
                {
                    name: "Base File Type",
                    extensions: inputFileExtensions,
                },
            ],
        });

        return file ?? undefined;
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
