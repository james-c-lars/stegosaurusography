<script lang="ts">
    import {save} from '@tauri-apps/api/dialog';
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined = undefined;
    export let dialogTitle: string | undefined = undefined;
    export let inputFileExtension: string | undefined = undefined;

    function selectFile(): Promise<string> {
        return save({
            title: dialogTitle,
            filters: [
                {
                    name: "Base File Type",
                    extensions: [inputFileExtension]
                }
            ]
        }).then(file => file as string)
    }
</script>

<FileInput bind:dialogTitle={dialogTitle} bind:selectedFile={selectedFile} selectFunction={selectFile}/>

