<script>
    import { invoke } from "@tauri-apps/api";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { songQueue, currentSongIndex } from "../../lib/stores/queue.js";
</script>

<section class="song-queue">
    <p>Song queue</p>
    {#if $songQueue}
        <ol class="queue-list">
            {#each $songQueue as song}
                {#if song.track_number > $currentSongIndex}
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