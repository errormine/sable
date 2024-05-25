<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { songQueue, currentSongIndex } from "../stores/audioPlayer";
    import { sec2time } from "../utils";
    import Window from "../comp/Window.svelte";
</script>

<Window title="SONG QUEUE">
    <section class="song-queue">
        {#if $songQueue.length > 0}
        <ol class="queue-list">
            {#each $songQueue as song, index}
            {#if index > $currentSongIndex}
            <li class="song-item">
                <img src={convertFileSrc(song.cover_path)} alt="">
                <section class="no-wrap">
                    <p class="title no-wrap" title={song.title}><strong>{song.title}</strong></p>
                    <p class="artist no-wrap" title={song.artist}>{song.artist}</p>
                </section>
                <p class="duration no-wrap">{sec2time(song.duration)}</p>
            </li>
            {/if}
            {/each}
        </ol>
        {:else}
        <p>Empty</p>
        {/if}
    </section>
</Window>

<style>
    .song-queue {
        background: var(--clr-gray-1);
        padding: 0 0.5rem;
        overflow-y: auto;
    }

    .queue-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .song-item {
        display: grid;
        grid-template-columns: auto 1fr 5ch;
        align-items: center;
        gap: 0.5rem;
    }

    .song-item img {
        width: 3rem;
        height: 3rem;
        border-radius: 0.25rem;
    }

    .song-item .duration {
        text-align: right;
    }
</style>