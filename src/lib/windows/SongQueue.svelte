<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { songQueue, currentSongIndex } from "../stores/audioPlayer";
    import { sec2time } from "../utils";
    import Window from "../comp/Window.svelte";

    let queue;
    let queueList;

    function scrollToCurrentSong() {
        if (queueList == null) return;
        let songItem = queueList.children[$currentSongIndex];
        if (songItem == null) return;
        // @ts-ignore
        queue.scrollTo(0, songItem.offsetTop - queue.clientHeight / 2 + songItem.clientHeight / 2);
    }

    currentSongIndex.subscribe(() => {
        scrollToCurrentSong();
    });

    songQueue.subscribe(() => {
        scrollToCurrentSong();
    });

    onMount(() => {
        addEventListener('resize', scrollToCurrentSong);
    });
</script>

<Window title="SONG QUEUE">
    <section class="song-queue" bind:this={queue}>
        {#if $songQueue.length > 0}
            <ol bind:this={queueList} class="queue-list">
                {#each $songQueue as song, index}
                <li class="song-item" class:active={index == $currentSongIndex}>
                    <button>
                        <img src={convertFileSrc(song.cover_path)} alt="">
                        <section class="no-wrap">
                            <p class="title no-wrap" title={song.title}><strong>{song.title}</strong></p>
                            <p class="artist no-wrap" title={song.artist}>{song.artist}</p>
                        </section>
                        <p class="duration no-wrap">{sec2time(song.duration)}</p>
                    </button>
                </li>
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
        scroll-behavior: smooth;
    }

    .queue-list {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .song-item {
        color: var(--clr-gray-7);
        border-radius: 0.25rem;
        overflow: hidden;
    }

    .song-item.active {
        color: var(--clr-gray-9);
        background: var(--clr-gray-3);
    }

    .song-item button {
        display: grid;
        grid-template-columns: auto 1fr 5ch;
        align-items: center;
        gap: 0.5rem;
        padding: 0.25rem;
        width: 100%;
        text-align: left;
        color: inherit;
    }

    img {
        width: 3rem;
        height: 3rem;
        border-radius: 0.25rem;
    }

    .song-item .duration {
        text-align: right;
    }
</style>