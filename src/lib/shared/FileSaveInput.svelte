<script lang="ts">
    import { save } from "@tauri-apps/api/dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined = undefined;
    export const dialogTitle = "Save File";
    export const inputFileExtension = "";

    function selectFunction(): Promise<string> {
        return save({
            title: dialogTitle,
            filters: [
                {
                    name: "Base File Type",
                    extensions: [inputFileExtension],
                },
            ],
        }).then((file) => file);
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
