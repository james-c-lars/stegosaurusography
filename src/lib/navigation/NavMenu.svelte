<script lang="ts">
    import "../../routes/styles.css";

    export let open: boolean;

    enum Tab {
        Encode,
        Decode,
        About,
    }

    let currentTab: Tab = Tab.Encode;
</script>

<nav class="nav-menu" hidden={!open}>
    <a  href="/encoder" class="nav-button" hidden={!open} class:selected={currentTab === Tab.Encode}
        on:click={() => currentTab = Tab.Encode}>
        Encode
    </a>
    <a  href="/decoder" class="nav-button" hidden={!open} class:selected={currentTab === Tab.Decode}
        on:click={() => currentTab = Tab.Decode}>
        Decode
    </a>
    <a  href="/about" class="nav-button" hidden={!open} class:selected={currentTab === Tab.About}
        on:click={() => currentTab = Tab.About}>
        About
    </a>
</nav>

<style>
    .nav-menu {
        width: var(--nav-menu-width);
        height: 100%;
        position: absolute;
        top: 0;
        left: calc(-1 * var(--nav-menu-width));

        padding: 0.5rem 0;

        display: flex;
        flex-direction: column;

        background: linear-gradient(180deg, var(--bg-high-contrast) 0%, var(--bg-low-contrast) 100%);
    }

    .nav-button {
        --left-padding: 1rem;
        --line-height: 2rem;

        position: relative;
        padding-left: var(--left-padding);

        color: var(--text-deemphasized);
        text-decoration-line: none;
        line-height: var(--line-height);
    }

    .nav-button.selected {
        color: var(--text-color);
    }

    .nav-button.selected::before {
        content: '';

        --diameter: 0.375rem;

        width: var(--diameter);
        aspect-ratio: 1;
        position: absolute;
        left: calc((var(--left-padding) - var(--diameter)) / 2);
        top: calc((var(--line-height) - var(--diameter)) / 2);

        border-radius: 100%;
        background-color: var(--accent-33);
    }

    .nav-button:hover {
        background: var(--accent-subtle);
        color: var(--text-color);
    }
</style>
