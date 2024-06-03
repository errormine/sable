<svelte:options accessors />
<script>
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event';
    import { open } from '@tauri-apps/api/dialog';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { invokeWithToast } from '../utils';
    import { addToast } from '../stores/notifications';
    import IonIosClose from 'virtual:icons/ion/ios-close';
    import { sec2time } from '../utils';
    import { addToQueue, currentSong, insertIntoQueue, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';
    import { loadSongs } from '../stores/songLibrary';
    import PopoutWindow from './PopoutWindow.svelte';
    import AlbumCover from './AlbumCover.svelte';
    import IconButton from './IconButton.svelte';

    export let domNode = null;
    export let activeAlbum;
    export let songList = [];

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

    let songEditDialog;
    let songContextMenu;
    let selectedSong;

    function showSongMenu(e, song) {
        songContextMenu.show(e);
        selectedSong = song;
    }

    function playSongAndQueue(song, offset) {
        selectedSong = song;
        play(song);
        setQueue(songList, offset);
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

    // TODO: this logic is duplicated in AlbumSelector.svelte and should probably be moved to a shared location
    function openEditDialog() {
        songEditDialog.showModal();
    }

    function closeEditDialog() {
        selectedSong = null;
        songEditDialog.close();
    }

    async function getNewCover() {
        const coverPath = await open({
            filters: [{ name: 'Images', extensions: ['jpg', 'png'] }],
            multiple: false
        });

        if (coverPath) {
            songEditDialog.querySelector('#cover-path').value = coverPath;
            selectedSong.cover_path = coverPath;
        }
    }

    async function updateSong() {
        let formData = new FormData(songEditDialog.querySelector('form'));

        await invokeWithToast('update_metadata_song', {
            locationOnDisk: activeAlbum.location_on_disk,
            filePath: selectedSong.file_path,
            coverPath: formData.get('cover-path'),
            title: formData.get('title'),
            artist: formData.get('artist'),
            albumTitle: formData.get('album-title'),
            albumArtist: formData.get('album-artist'),
            trackNumber: Number(formData.get('track-number')),
            discNumber: Number(formData.get('disc-number')),
            year: Number(formData.get('year')),
            genre: formData.get('genre')
        });

        closeEditDialog();
        songList = await loadSongs(activeAlbum);
    }
</script>

<PopoutWindow bind:dialog={songEditDialog} title={selectedSong ? selectedSong.title : ""} onClose={closeEditDialog}>
    {#if selectedSong}
        <form class="item-edit-form">
            <fieldset>
                <label for="cover-path"><AlbumCover path={selectedSong.cover_path}/></label>
                <input on:click={getNewCover} hidden type="text" id="cover-path" name="cover-path">
                <p><strong>WARNING: Tag editing has not been thoroughly tested! Please let me know if you run into issues.</strong></p>
            </fieldset>

            <fieldset>
                <label for="title">
                    <span>Title</span>
                    <input type="text" id="title" name="title" value={selectedSong.title}>
                </label>
        
                <label for="artist">
                    <span>Artist</span>
                    <input type="text" id="artist" name="artist" value={selectedSong.artist}>
                </label>
        
                <label for="album">
                    <span>Album Title</span>
                    <input type="text" id="album-title" name="album-title" value={selectedSong.album_title}>
                </label>
        
                <label for="album-artist">
                    <span>Album Artist</span>
                    <input type="text" id="album-artist" name="album-artist" value={selectedSong.album_artist}>
                </label>
        
                <label for="track-number">
                    <span>Track Number</span>
                    <input type="number" id="track-number" name="track-number" value={selectedSong.track_number}>
                </label>
        
                <label for="disc-number">
                    <span>Disc Number</span>
                    <input type="number" id="disc-number" name="disc-number" value={selectedSong.disc_number}>
                </label>
        
                <label for="year">
                    <span>Year</span>
                    <input type="number" id="year" name="year" value={selectedSong.year}>
                </label>
        
                <label for="genre">
                    <span>Genre</span>
                    <input type="text" id="genre" name="genre" value={selectedSong.genre}>
                </label>
            </fieldset>
            
            <fieldset class="footer">
                <button type="button" on:click={updateSong}>Save</button>
            </fieldset>
        </form>
    {/if}
</PopoutWindow>
<section bind:this={domNode} class="song-selector" class:hidden={!activeAlbum}>
    {#if activeAlbum}
        <section class="album-info-wrapper glass">
            <AlbumCover path={activeAlbum.cover_path} />
            <section class="songs">
                <header class="mb-05">
                    <h2>{activeAlbum.title}</h2>
                    <p class="subtitle">{activeAlbum.artist}</p>
                </header>
                <ol class="song-list">
                    {#each songList as song, index}
                        <li class="song-item">
                            <!-- TODO: I would like to have the ability to select multiple songs at once for metadata editing -->
                            <button class="song" title={song.title} 
                                class:active={selectedSong == song}
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
        </section>
        <ContextMenu bind:this={songContextMenu}>
            <Item on:click={playSelectedSongNext}>Play Next</Item>
            <Item on:click={addSelectedToQueue}>Add to Queue</Item>
            <Divider />
            <Item on:click={openEditDialog}>Edit</Item>
            <Item on:click={removeSelectedSong}>Remove</Item>
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
    
        & .track-number {
            color: var(--clr-gray-7);
        }
        
        & .duration {
            color: var(--clr-gray-7);
            text-align: right;
        }
    }
</style>