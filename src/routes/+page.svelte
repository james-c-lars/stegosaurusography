<script lang="ts">
    import { goto } from "$app/navigation";

    // Defaulting to the encoder when opening the app
    goto("/encoder").catch(() => {
        console.error("cannot go to default path");
    });
</script>
