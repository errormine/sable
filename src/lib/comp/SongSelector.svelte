<svelte:options accessors />
<script>
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import IonIosClose from 'virtual:icons/ion/ios-close';
    import { sec2time } from '../utils';
    import { addToQueue, currentSong, insertIntoQueue, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';
    import { addToast } from '../stores/notifications';

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

    let songEditDialog;
    let songContextMenu;
    let selectedSong;

    function showSongMenu(e, song) {
        songContextMenu.show(e);
        selectedSong = song;
    }

    function playSongAndQueue(song, offset) {
        play(song);
        setQueue($songList, offset);
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

    function showEditDialog() {
        let title = songEditDialog.querySelector('#title');
        let artist = songEditDialog.querySelector('#artist');
        let album = songEditDialog.querySelector('#album');

        title.value = selectedSong.title;
        artist.value = selectedSong.artist;
        album.value = selectedSong.album;

        songEditDialog.showModal();
    }

    async function updateSong() {
        let songDir = selectedSong.file_path.split('/').slice(0, -1).join('/');
        let formData = new FormData(songEditDialog.querySelector('form'));

        await invoke('update_song_info', {
            filePath: selectedSong.file_path,
            title: formData.get('title'),
            artist: formData.get('artist'),
            album: formData.get('album'),
        }).then(async (result) => {
            if (result != "success") {
                addToast({ message: "Failed to update song info", type: "error", timeout: 3000 });
                return;
            }
            await invoke('register_songs', { dir: songDir});
        });

        songEditDialog.close();
    }
</script>

<dialog bind:this={songEditDialog} class="song-editor">
    <header>
        <p>Edit</p>
        <button on:click={() => songEditDialog.close()}><IonIosClose/></button>
    </header>
    <form>
        <label for="title">Title</label>
        <input type="text" id="title" name="title">

        <label for="artist">Artist</label>
        <input type="text" id="artist" name="artist">

        <label for="album">Album</label>
        <input type="text" id="album" name="album">

        <label for="year">Year</label>
        <input type="number" id="year" name="year">
        
        <button type="button" on:click={updateSong}>Save</button>
    </form>
</dialog>
<section bind:this={domNode} class="album-info" class:hidden={$activeAlbum == null}>
    <section class="album-info-wrapper">
        {#if $songList && $activeAlbum != null}
            <img class="album-cover" src={convertFileSrc($activeAlbum.cover_path)} alt="">
            <section class="song-selector">
                <header class="mb-05">
                    <h2>{$activeAlbum.title}</h2>
                    <p class="subtitle">{$activeAlbum.artist}</p>
                </header>
                <ol class="song-list">
                    {#each $songList as song, index}
                        <li class="song-item">
                            <!-- so long!!!! -->
                            <button class="song" title={song.title} 
                                class:active={$currentSong.title == song.title && $currentSong.artist == song.artist}
                                on:click={() => playSongAndQueue(song, index)}
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
    </section>
    <ContextMenu bind:this={songContextMenu}>
        <Item on:click={playSelectedSongNext}>Play Next</Item>
        <Item on:click={addSelectedToQueue}>Add to Queue</Item>
        <Divider />
        <Item on:click={showEditDialog}>Edit</Item>
        <Item on:click={removeSelectedSong}>Remove</Item>
        <Divider />
        <Item>Open File Location</Item>
    </ContextMenu>
</section>

<style>
    .album-info {
        position: relative;
        padding: 0 0.5rem;
        margin-top: 1rem;
        
        & .album-info-wrapper {
            display: grid;
            grid-template-columns: 16rem 1fr;
            padding: 1rem;
            gap: 1rem;
            background: var(--clr-gray-3);
            border-radius: 0.5rem;
        }
    
        & .album-cover {
            border-radius: 0.25rem;
        }
    }

    .song-list {
        column-count: auto;
        column-width: 22vw;
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
            background: var(--clr-gray-0);
        }
        
        &:hover {
            background: var(--clr-gray-5);
        }
    
        & .track-number {
            color: var(--clr-gray-7);
        }
        
        & .duration {
            color: var(--clr-gray-7);
            text-align: right;
        }
    }
</style>