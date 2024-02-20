<script lang="ts">
    import "../../routes/styles.css";

    // This lets the <NavMenu> know whether it should be visually seen by the user
    // Also impacts whether the menu is interactable
    export let open: boolean;

    // This represents which tab is currently selected from the menu
    enum Tab {
        Encode,
        Decode,
        About,
    }

    let currentTab: Tab = Tab.Encode;
</script>

<nav class="nav-menu" hidden={!open}>
    <a
        href="/encoder"
        class="nav-button"
        hidden={!open}
        class:selected={currentTab === Tab.Encode}
        on:click={() => (currentTab = Tab.Encode)}
    >
        Encode
    </a>
    <a
        href="/decoder"
        class="nav-button"
        hidden={!open}
        class:selected={currentTab === Tab.Decode}
        on:click={() => (currentTab = Tab.Decode)}
    >
        Decode
    </a>
    <a
        href="/about"
        class="nav-button"
        hidden={!open}
        class:selected={currentTab === Tab.About}
        on:click={() => (currentTab = Tab.About)}
    >
        About
    </a>
    <!-- TODO: Create a page for tweaking the appearance of the app. Switching between dark and light mode, tweaking --accent-hue in CSS. -->
</nav>

<style>
    /* The root element */
    .nav-menu {
        /* Positioning the nav menu immediately to the left of the left margin of its parent */
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

    /* Each <a> in the menu */
    .nav-button {
        /* Used for later calculations for centering the selected accent shape */
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

    /* This pseudo element is the visual accent mark that shows in front of the selected <a> */
    .nav-button.selected::before {
        content: "";

        /* Used in math for centering the selection indicator */
        --diameter: 0.375rem;

        width: var(--diameter);
        aspect-ratio: 1;
        position: absolute;
        /* Visually centering the element on the left of its parent <a> */
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
