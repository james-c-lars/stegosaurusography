<script lang="ts">
    import AppBar from "$lib/app_bar/AppBar.svelte";
    import NavMenu from "$lib/navigation/NavMenu.svelte";
    import "./styles.css";

    // Represents whether the menu is toggled open or close
    // Used for state management
    let navMenuOpen: boolean = false;
    // Represents whether any part of the menu is visible (due to CSS transitions)
    // Used for determining whether the menu elements should be hidden (in the tab order)
    let navMenuVisible: boolean = navMenuOpen;

    // The main element encapsulating our app
    let appView: HTMLElement;

    // Triggers after every transition in our app ends
    function transitionend(event: Event) {
        // If the transition is due to our root element (i.e. is the menu opening or closing)
        if (event.target === appView && !navMenuOpen) {
            // If the state of the menu is closed and the transition just ended, the menu
            // has finished closing and should now be hidden and not part of the tab order
            navMenuVisible = false;
        }
    }

    // Triggers before every transition in our app starts
    function transitionstart(event: Event) {
        // If the transition is due to our root element (i.e. is the menu opening or closing)
        if (event.target === appView && navMenuOpen) {
            // If the state of the menu is open and the transition just started, the menu
            // is beginning to open and should no longer be hidden
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
    /* The root element */
    .app-view {
        /* We need consistency between <NavMenu> and this for the visuals of opening and
            closing the menu */
        --nav-menu-width: 6rem;

        height: 100%;
        position: relative;

        /* <NavMenu> handles its own positioning, so this is just for <AppBar> and <slot> */
        display: flex;
        flex-direction: column;

        /* Used to shift over the body of our app when <NavMenu> opens */
        transition: margin-left 0.5s ease-in-out;
    }

    .app-view.nav-menu-open {
        margin-left: var(--nav-menu-width);
    }
</style>
