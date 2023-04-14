<script lang="ts">
    import { save } from "@tauri-apps/api/dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined = undefined;
    export let dialogTitle = "Save File";
    export let inputFileExtension = "";

    function selectFunction(): Promise<string | undefined> {
        return save({
            title: dialogTitle,
            filters: [
                {
                    name: "Base File Type",
                    extensions: [inputFileExtension],
                },
            ],
        }).then((file) => file ?? undefined);
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
