<script lang="ts">
    import AppBar from "$lib/app_bar/AppBar.svelte";
    import NavMenu from "$lib/navigation/NavMenu.svelte";
    import "./styles.css";

    let navMenuOpen: boolean;
</script>

<main class="app-view">
    <AppBar bind:navMenuOpen={navMenuOpen} />
    <NavMenu bind:open={navMenuOpen} />
    <slot />
</main>

<style>
    .app-view {
        height: 100%;

        display: grid;
        grid-template-columns: auto 1fr;
        grid-template-rows: auto 1fr;
    }

    :global(.app-view > *:first-child) {
        grid-column: span 2;
    }

    :global(.app-view > *:last-child) {
        grid-column: 2;
    }
</style>
