<script context="module">
    import { writable } from "svelte/store";

    export let currentlyPlaying = writable(null);
</script>

<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
</script>

<section class="track-info">
    <p>Track Information</p>
    {#if $currentlyPlaying}
            <section>
                <p>Title: {$currentlyPlaying.title}</p>
                <p>Artist: {$currentlyPlaying.artist}</p>
            </section>
            <img src={convertFileSrc($currentlyPlaying.file_path.replace(/[^/\\]*$/, 'Cover.jpg'))} alt="">
    {:else}
        <p>Empty</p>
    {/if}
</section>

<style>
    .track-info {
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
</style>