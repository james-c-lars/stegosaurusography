<script lang="ts">
    import "../../routes/styles.css";

    export let navMenuOpen: boolean = false;

    let holdNavMenuOpen: boolean = false;
    let hoveringNavMenu: boolean = false;

    function click() {
        holdNavMenuOpen = !holdNavMenuOpen;
        navMenuOpen = holdNavMenuOpen;
    }

    function mouseenter() {
        hoveringNavMenu = true;
        navMenuOpen = holdNavMenuOpen || hoveringNavMenu;
    }

    function mouseleave() {
        hoveringNavMenu = false;
        navMenuOpen = holdNavMenuOpen || hoveringNavMenu;
    }
</script>

<div class="app-bar">
    <button id="menu-button" on:click={click} on:mouseenter={mouseenter} on:mouseleave={mouseleave}>Menu</button>
    <button>Dark Mode</button>
</div>

<style>
    .app-bar {
        display: flex;
        justify-content: space-between;
    }
</style>
