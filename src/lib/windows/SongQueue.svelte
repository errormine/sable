<script>
    import { onMount } from "svelte";
    import { songQueue, currentSongIndex, jumpToSong } from "../stores/audioPlayer";
    import { sec2time } from "../utils";
    import Window from "../comp/Window.svelte";
    import AlbumCover from "../comp/AlbumCover.svelte";
    import CardListItem from "../comp/CardListItem.svelte";

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
                    <!-- This is actually unreadable I'm so sorry -->
                    <CardListItem 
                        title={song.title} 
                        subtitle={song.artist} 
                        floatingText={sec2time(song.duration)} 
                        onClick={() => jumpToSong(index)} 
                        highlighted={index == $currentSongIndex}
                            >
                        <AlbumCover path={song.cover_path} />
                    </CardListItem>
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
</style>