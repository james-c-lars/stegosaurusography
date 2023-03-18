<script lang="ts">
    export const dialogTitle = "File Input";
    export let selectedFile: string | undefined = undefined;
    export const selectFunction: (() => Promise<string>) | undefined =
        undefined;

    async function launchSelectDialog() {
        if (selectFunction) {
            selectedFile = await selectFunction();
        }
    }
</script>

<div>
    <h2>{dialogTitle}</h2>
    <button on:click={launchSelectDialog}>Select a File</button>
</div>
