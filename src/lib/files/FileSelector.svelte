<script lang="ts">
    export let title = "File";

    let selected_file: string | undefined = undefined;
</script>

<div>
    <div>
        <span>{title}</span>
        <span></span>
        <button>X</button>
    </div>

    <div>
        {#if selected_file}
            <div>
                <img src={selected_file} alt="The selected file" />
                <div>{selected_file}</div>
            </div>
        {:else}
            <div>
                <div></div>
                <div>+</div>
                <div>Select a file</div>
                <div></div>
            </div>
        {/if}
    </div>
</div>

<style>
</style>
