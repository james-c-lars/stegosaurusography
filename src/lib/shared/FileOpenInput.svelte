<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined = undefined;
    export const dialogTitle = "Open File";

    function selectFunction(): Promise<string> {
        return open({
            title: dialogTitle,
            filters: [
                {
                    name: "Image",
                    extensions: ["jpg", "jpeg", "png", "svg"],
                },
            ],
        }).then((file) => file as string);
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
