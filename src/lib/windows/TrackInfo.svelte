<script context="module">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { get, writable } from "svelte/store";

    export let currentlyPlaying = writable({
        title: "",
        artist: "",
        file_path: "",
        cover_path: "",
        duration: 0,
    });
</script>

<section class="track-info">
    <p>Track Information</p>
    {#if $currentlyPlaying.title != '' && $currentlyPlaying.artist != ''}
            <section>
                <p>{$currentlyPlaying.title}</p>
                <p>{$currentlyPlaying.artist}</p>
            </section>
            <img src={convertFileSrc($currentlyPlaying.cover_path)} alt="">
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