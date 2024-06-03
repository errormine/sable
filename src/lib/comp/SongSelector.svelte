<svelte:options accessors />
<script>
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { invokeWithToast } from '../utils';
    import IonIosClose from 'virtual:icons/ion/ios-close';
    import IonVolumeHigh from 'virtual:icons/ion/volume-high';
    import { sec2time } from '../utils';
    import { addToQueue, currentSong, insertIntoQueue, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';
    import AlbumCover from './AlbumCover.svelte';
    import IconButton from './IconButton.svelte';
    import { openEditDialog, selectedSongs } from '../stores/tagEditor';
    import { loadSongs, refreshSongList, openAlbum, songList } from '../stores/songLibrary';

    export let domNode = null;

    let owner = null;
    
    export function updateSize(offsetNode) {
        // Hack to keep the song selector the correct size
        let difference = owner.offsetLeft;
        domNode.style.left = -offsetNode.offsetLeft + difference + 'px';
        domNode.style.width = owner.clientWidth + 'px';
    }
    
    onMount(async () => {
        owner = domNode.parentNode;
        
        addEventListener('resize', () => {
            // TODO: fix errors when parent node doesn't exist
            updateSize(domNode.parentNode);
        });
    });

    let songContextMenu;

    function showContextMenu(e, song) {
        if (!$selectedSongs.includes(song)) {
            $selectedSongs = [song];
        }
        songContextMenu.show(e);
    }

    let lastSelectedIndex;

    function windowKeyDown(e) {
        if (e.key === 'Escape') {
            $selectedSongs = [];
        }
    }

    function select(e, song, index) {
        if (e.shiftKey && lastSelectedIndex !== undefined) {
            let start = Math.min(lastSelectedIndex, index);
            let end = Math.max(lastSelectedIndex, index);
            $selectedSongs = $songList.slice(start, end + 1);
        } else if (e.ctrlKey && !$selectedSongs.includes(song)) {
            $selectedSongs = [...$selectedSongs, song];
        } else if (e.ctrlKey && $selectedSongs.includes(song)) {
            $selectedSongs = $selectedSongs.filter(s => s !== song);
        } else if ($selectedSongs == [song]) {
            $selectedSongs = [];
        } else {
            $selectedSongs = [song];
        }

        lastSelectedIndex = index;
    }

    function playSongAndQueue(song, offset) {
        play(song);
        setQueue($songList, offset);
    }

    function playSelectedNext() {
        insertIntoQueue($selectedSongs);
    }

    function addSelectedToQueue() {
        addToQueue($selectedSongs);
    }

    async function removeSelected() {
        for (let song of $selectedSongs) {
            await invoke('remove_song', song);
        }
        refreshSongList($openAlbum);
    }
</script>

<svelte:window on:keydown={windowKeyDown} />
<section bind:this={domNode} class="song-selector" class:hidden={!$openAlbum || $songList.length < 1}>
    {#if $openAlbum}
        <section class="album-info-wrapper glass">
            <AlbumCover path={$openAlbum.cover_path} />
            <section class="songs">
                <header class="mb-05">
                    <h2>{$openAlbum.title}</h2>
                    <p class="subtitle">{$openAlbum.artist}</p>
                </header>
                <ol class="song-list">
                    {#each $songList as song, index}
                        <li class="song-item">
                            <button class="song" title={song.title} 
                                class:active={$currentSong.title == song.title && $currentSong.artist == song.artist}
                                class:selected={$selectedSongs.includes(song)}
                                on:click={(e) => select(e, song, index)}
                                on:dblclick={() => playSongAndQueue(song, index)}
                                on:contextmenu={(e) => showContextMenu(e, song)}>
                                <span class="track-number">
                                    {#if $currentSong.title == song.title && $currentSong.artist == song.artist}
                                        <IonVolumeHigh />
                                    {:else}
                                        {song.track_number}
                                    {/if}
                                </span>
                                <p class="song-title no-wrap">{song.title}</p>
                                <span class="duration">{sec2time(song.duration)}</span>
                            </button>
                        </li>
                    {/each}
                </ol>
            </section>
        </section>
        <ContextMenu bind:this={songContextMenu}>
            <Item on:click={playSelectedNext}>Play Next</Item>
            <Item on:click={addSelectedToQueue}>Add to Queue</Item>
            <Divider />
            <Item on:click={openEditDialog}>Edit</Item>
            <Item on:click={removeSelected}>Remove</Item>
            <Divider />
            <Item>Open File Location</Item>
        </ContextMenu>
    {/if}
</section>

<style>
    .song-selector {
        position: relative;
        padding: 0 1rem;
        margin: 1rem 0;

        & .album-info-wrapper {
            display: grid;
            grid-template-columns: 16rem 1fr;
            padding: 1rem;
            gap: 1rem;
            border-radius: 0.5rem;
        }
    
        & .album-cover {
            border-radius: 0.25rem;
        }
    }

    .song-list {
        column-count: auto;
        column-width: 20rem;
        column-gap: 3rem;
    }
    
    .song {
        display: grid;
        grid-template-columns: 3ch 1fr auto;
        padding: 0.35rem 0.5rem;
        border-radius: 0.25rem;
        gap: 1.5rem;
        transition: all 200ms;
        width: 100%;
    
        &.active {
            background: var(--clr-gray-3);
        }

        &.selected {
            background: var(--clr-gray-5);
        }
    
        & .track-number {
            height: 1rem;
            color: var(--clr-gray-7);
        }
        
        & .duration {
            color: var(--clr-gray-7);
            text-align: right;
        }
    }
</style>