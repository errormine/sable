<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { songQueue, currentSongIndex } from "../stores/audioPlayer";
</script>

<section class="song-queue">
    <p>Song queue</p>
    {#if $songQueue.length > 0}
        <ol class="queue-list">
            {#each $songQueue as song, index}
                {#if index > $currentSongIndex}
                    <li class="song-item">
                        <!-- Evil!!! -->
                        <img src={convertFileSrc(song.cover_path)} alt="">
                        <section class="no-wrap">
                            <p class="title no-wrap"><strong>{song.title}</strong></p>
                            <p class="artist no-wrap">{song.artist}</p>
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
    .queue-list {
        height: 40vh;
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