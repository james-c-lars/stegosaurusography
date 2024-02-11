<script lang="ts">
    import "../../routes/styles.css";

    export let open: boolean;
</script>

<nav class="nav-menu" class:open={open}>
    <a href="/encoder" class="nav-button">Encode</a>
    <a href="/decoder" class="nav-button">Decode</a>
    <a href="/about" class="nav-button">About</a>
</nav>

<style>
    .nav-menu {
        display: none;
        flex-direction: column;
    }

    .nav-button:hover {
        background: mediumaquamarine;
    }

    .nav-menu.open,
    .nav-menu:hover
     {
        display: flex;
    }
</style>
