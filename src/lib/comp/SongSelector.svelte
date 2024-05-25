<svelte:options accessors />
<script>
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { sec2time } from '../utils';
    import { addToQueue, currentSong, insertIntoQueue, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';

    export let domNode = null;

    const activeAlbum = getContext('activeAlbum');
    const songList = getContext('songList');

    let owner = null;
    
    export function updateSize(offsetNode) {
        // Hack to keep the song selector the correct size
        let difference = owner.offsetLeft;
        domNode.style.left = -offsetNode.offsetLeft + difference + 'px';
        domNode.style.width = owner.clientWidth + 'px';
    }
    
    onMount(() => {
        owner = domNode.parentNode;

        addEventListener('resize', () => {
            // TODO: fix errors when parent node doesn't exist
            updateSize(domNode.parentNode);
        });
    });

    let songContextMenu;
    let selectedSong;

    function showSongMenu(e, song) {
        songContextMenu.show(e);
        selectedSong = song;
    }

    function playSongAndQueue(song) {
        play(song);
        setQueue($songList, song.track_number);
    }

    function playSelectedSongNext() {
        insertIntoQueue(selectedSong);
    }

    function addSelectedToQueue() {
        addToQueue([selectedSong]);
    }

    async function removeSelectedSong() {
        await invoke('remove_song', selectedSong);
    }
</script>

<section bind:this={domNode} class="song-selector" class:hidden={$activeAlbum == null}>
    {#if $songList && $activeAlbum != null}
        <img class="large-album-cover" src={convertFileSrc($activeAlbum.cover_path)} alt="">
        <section class="album-info">
            <header class="mb-05">
                <h2>{$activeAlbum.title}</h2>
                <p class="subtitle">{$activeAlbum.artist}</p>
            </header>
            <ol class="song-list">
                {#each $songList as song}
                    <li class="song">
                        <!-- so long!!!! -->
                        <button title={song.title} 
                            class:active={$currentSong.title == song.title && $currentSong.artist == song.artist}
                            on:click={() => playSongAndQueue(song)}
                            on:contextmenu={(e) => showSongMenu(e, song)}>
                            <span class="track-number">{song.track_number}</span>
                            <p class="song-title no-wrap">{song.title}</p>
                            <span class="duration">{sec2time(song.duration)}</span>
                        </button>
                    </li>
                {/each}
            </ol>
        </section>
    {/if}
    <ContextMenu bind:this={songContextMenu}>
        <Item on:click={playSelectedSongNext}>Play Next</Item>
        <Item on:click={addSelectedToQueue}>Add to Queue</Item>
        <Divider />
        <Item>Edit</Item>
        <Item on:click={removeSelectedSong}>Remove</Item>
        <Divider />
        <Item>Open File Location</Item>
    </ContextMenu>
</section>

<style>
    .song-selector {
        position: relative;
        display: grid;
        grid-template-columns: 16rem 1fr;
        padding: 1rem;
        margin-top: 1rem;
        gap: 1rem;
        background: linear-gradient(white, var(--clr-gray-3));
        border-top: 1px solid var(--clr-gray-5);
        border-bottom: 1px solid var(--clr-gray-5);
    }

    .song-selector .song-list {
        column-count: auto;
        column-width: 22vw;
        column-gap: 3rem;
    }
    
    .song-selector .song button {
        display: grid;
        grid-template-columns: 3ch 1fr auto;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        gap: 1.5rem;
        transition: all 200ms;
        width: 100%;
    }
    
    .song-selector .song button:hover {
        opacity: 0.75;
    }

    :global(.song-selector .song button.active) {
        outline: 1px solid var(--clr-gray-5);
        background: linear-gradient(white, var(--clr-gray-3));
    }
    
    .song-selector .song * {
        display: inline;
        text-align: left;
    }
    
    .song .song-title {
        font-weight: normal;
    }

    .song .duration {
        float: right;
    }
</style>