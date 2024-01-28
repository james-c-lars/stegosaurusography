<script lang="ts">
    import Encoder from "./lib/encoder/Encoder.svelte";
    import NavMenu from "./lib/navigation/NavMenu.svelte";
</script>

<main class="app-view">
    <div class="nav-view">
        <NavMenu />
    </div>

    <div class="main-view">
        <div class="app-bar">
            <button>Menu</button>
            <span class="app-bar-divider"></span>
            <button>Dark Mode</button>
        </div>

        <div class="tool-view">
            <Encoder />
        </div>
    </div>
</main>

<style>
    .app-view {
        display: flex;
        flex-direction: row;
        height: 100%;
    }

    .nav-view {
        display: inline-block;
        height: 100%;
    }

    .main-view {
        display: inline-flex;
        flex-grow: 1;
        flex-direction: column;
    }

    .app-bar {
        display: flex;
    }

    .app-bar-divider {
        flex-grow: 1;
    }

    .tool-view {
        height: 100%;
        display: flex;
        flex-direction: row;
        flex-grow: 1;
    }
</style>
