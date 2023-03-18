<script lang="ts">
    export let dialogTitle: string | undefined = undefined;
    export let selectedFile: string | undefined = undefined;
    export let selectFunction: (() => Promise<string>) | undefined = undefined;

    async function launchSelectDialog() {
        selectedFile = await selectFunction();
    }
</script>

<div>
    <h2>{dialogTitle}</h2>
    <button on:click={launchSelectDialog}>Select a File</button>
</div>
