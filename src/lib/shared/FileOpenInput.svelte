<script lang="ts">
    import {open} from '@tauri-apps/api/dialog';
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined = undefined;
    export let dialogTitle: string | undefined = undefined;

    function selectFile(): Promise<string> {
        return open({
            title: dialogTitle,
            multiple: false,
            filters: [
                {
                    name: "Image",
                    extensions: [
                        "jpg",
                        "jpeg",
                        "png",
                        "svg"
                    ]
                }
            ]
        }).then(file => file as string);
    }
</script>

<FileInput bind:dialogTitle={dialogTitle} bind:selectedFile={selectedFile} selectFunction={selectFile}/>
