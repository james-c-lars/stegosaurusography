<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import FileInput from "./FileInput.svelte";

    export let selectedFile: string | undefined;
    export let dialogTitle = "Open File";

    async function selectFunction(): Promise<string> {
        let file = await open({
            title: dialogTitle,
            filters: [
                {
                    name: "Image",
                    extensions: ["jpg", "jpeg", "png", "svg", "txt"],
                },
            ],
        });

        return file as string;
    }
</script>

<FileInput {dialogTitle} bind:selectedFile {selectFunction} />
