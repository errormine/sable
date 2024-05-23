<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let songs;
    let currentSongIndex;

    export function fresh(newSongs, offset = 1) {
        songs = newSongs;
        currentSongIndex = offset;

        // track numbers are 1-indexed
        dispatch('playSong', songs[currentSongIndex - 1]);
    }
</script>

<section class="song-queue">
    <p>Song queue</p>
    {#if songs}
        <ol class="queue-list">
            {#each songs as song}
                {#if song.track_number > currentSongIndex}
                    <li class="song-item">
                        <!-- Evil!!! -->
                        <img src={convertFileSrc(song.file_path.replace(/[^/\\]*$/, 'Cover.jpg'))} alt="">
                        <section class="no-wrap">
                            <p class="no-wrap">{song.title}</p>
                            <p class="no-wrap">{song.artist}</p>
                        </section>
                    </li>
                {/if}
            {/each}
        </ol>
    {:else}
        <p>Empty</p>
    {/if}
</section>

<style>
    .song-queue {
        max-height: 50vh;
        overflow-y: auto;
    }

    .song-item {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 0.5rem;
        padding: 0.25rem;
    }

    .song-item img {
        width: 3rem;
        height: 3rem;
        border-radius: 0.25rem;
    }
</style>