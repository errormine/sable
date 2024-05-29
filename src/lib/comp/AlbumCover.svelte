<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    export let path = "";
</script>

{#if path == ""}
    <img src="/assets/placeholder/cover.png" alt="" width="128" height="128" loading="lazy">
{:else}
    <img src={convertFileSrc(path)} alt="" width="128" height="128" loading="lazy">
{/if}

<style>
    img {
        width: 100%;
        border-radius: 0.25rem;
    }
</style>