<script lang="ts">
    import Encoder from "./lib/encoder/Encoder.svelte";
</script>

<main class="view">
    <h1>Stegasaurusography, the most dino-mic way to transfer messages!</h1>

    <div class="row">
        <Encoder/>
    </div>
</main>

<style>
    .view {
        flex: auto;
        flex-grow: 1;
    }
</style>
