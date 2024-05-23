<script context="module">
    import { get, writable } from "svelte/store";
    import { play } from "./PlayerControls.svelte";

    let songQueue = writable([]);
    let currentSongIndex = 0;

    export function setQueue(songs, offset) {
        songQueue.set(songs);
        currentSongIndex = 0;

        // track numbers are 1-indexed
        while (currentSongIndex < offset) {
            currentSongIndex++;
            songQueue.update((queue) => queue.slice(1));
        }
    }

    export async function attemptPlayNext() {
        currentSongIndex++;
        songQueue.update((queue) => queue.slice(1));
        let nextSong = get(songQueue)[0];
        if (!nextSong) return;
        play(nextSong);
    }
</script>

<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
</script>

<section class="song-queue">
    <p>Song queue</p>
    {#if $songQueue.length > 0}
        <ol class="queue-list">
            {#each $songQueue as song}
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