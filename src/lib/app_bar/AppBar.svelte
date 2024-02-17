<script lang="ts">
    import "../../routes/styles.css";

    export let navMenuOpen: boolean;
</script>

<div class="app-bar">
    <!-- TODO: Make these icons, rather than text -->
    <button id="menu-button" on:click={() => navMenuOpen = !navMenuOpen}>Menu</button>
    <button>Dark Mode</button>
</div>

<style>
    /* The root element */
    .app-bar {
        display: flex;
        justify-content: space-between;
    }
</style>
