<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    let songs;
    let currentSongIndex = 0;

    export function fresh(newSongs, offset = 0) {
        songs = newSongs;
        currentSongIndex = offset;
    }
</script>

<section class="song-queue">
    {#if songs}
        <ol class="queue-list">
            {#each songs as song}
                <li class="song-item">
                    <!-- Evil!!! -->
                    <img src={convertFileSrc(song.file_path.replace(/[^/\\]*$/, 'Cover.jpg'))} alt="">
                    <section class="no-wrap">
                        <p class="no-wrap">{song.title}</p>
                        <p class="no-wrap">{song.artist}</p>
                    </section>
                </li>
            {/each}
        </ol>
    {:else}
        <p>No songs in queue</p>
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