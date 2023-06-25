<script lang="ts">
    export let dialogTitle = "File Input";
    export let selectedFile: string | undefined;
    export let selectFunction: () => Promise<string | undefined>;

    async function launchSelectDialog() {
        selectedFile = await selectFunction();
    }
</script>

<div>
    <h2>{dialogTitle}</h2>
    <button on:click={launchSelectDialog}>Select a File</button>
</div>
