<script lang="ts">
    import AppBar from "$lib/app_bar/AppBar.svelte";
    import NavMenu from "$lib/navigation/NavMenu.svelte";
    import "./styles.css";

    let navMenuOpen: boolean = false;
    let navMenuVisible: boolean = navMenuOpen;

    let appView: HTMLElement;

    function transitionend(event: Event) {
        if (event.target === appView && !navMenuOpen) {
            navMenuVisible = false;
        }
    }

    function transitionstart(event: Event) {
        if (event.target === appView && navMenuOpen) {
            navMenuVisible = true;
        }
    }
</script>

<main class="app-view" class:nav-menu-open={navMenuOpen} bind:this={appView}
      on:transitionend={transitionend} on:transitionstart={transitionstart}>
    <AppBar bind:navMenuOpen={navMenuOpen} />
    <NavMenu bind:open={navMenuVisible} />
    <slot />
</main>

<style>
    .app-view {
        --nav-menu-width: 6rem;

        height: 100%;
        position: relative;

        display: flex;
        flex-direction: column;

        transition: margin-left 0.5s ease-in-out;
    }

    .app-view.nav-menu-open {
        margin-left: var(--nav-menu-width);
    }
</style>
