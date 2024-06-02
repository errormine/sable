<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { setQueue, addToQueue, attemptPlayNext, currentSong, beginPlayBack, togglePlayback } from '../stores/audioPlayer';
    import { loadSongs, refreshLibrary } from '../stores/songLibrary';
    import Album from '../comp/Album.svelte';
    import SongSelector from '../comp/SongSelector.svelte';
    import { downloadCoverImage, getAlbumImage } from '../stores/lastfmAPI';
    import PopoutWindow from './PopoutWindow.svelte';
    import AlbumCover from './AlbumCover.svelte';
    import { invokeWithToast } from '../utils';

    export let albums;

    $: albums, clearActiveAlbum();

    function clearActiveAlbum() {
        activeAlbum = null;
        songList = [];
    }

    let activeAlbum;
    let albumSelector;
    let songSelector;
    let songList;

    async function displayAlbumDetails(e, album) {
        let target = e.currentTarget;
        if (activeAlbum != album) {
            activeAlbum = album;
            songList = await loadSongs(album);
            
            // Show song selector
            let albumListItem = target.parentNode;
            albumListItem.appendChild(songSelector.domNode);
            songSelector.updateSize(albumListItem);
            albumSelector.scrollTo(0, target.offsetTop - 40);
        } else {
            // Ignore double clicks
            if (e.detail > 1) return;
            activeAlbum = null;
            songList = [];
        }
    }

    async function playAlbum(album) {
        setQueue(await loadSongs(album), -1);
        attemptPlayNext();
    }

    let albumContextMenu;
    let selectedAlbum;
    
    function showAlbumContextMenu(e, album) {
        albumContextMenu.show(e);
        selectedAlbum = album;
    }

    async function removeSelectedAlbum() {
        await invoke('remove_album', { album: selectedAlbum.title, artist: selectedAlbum.artist });
        await refreshLibrary();
    }

    async function playSelectedAlbumNext() {
        // TODO: Make this just add the album after the current song
        setQueue(await loadSongs(selectedAlbum));
        if ($currentSong.title == '') {
            attemptPlayNext();
        }
    }

    async function addSelectedToQueue() {
        addToQueue(await loadSongs(selectedAlbum));
    }

    async function downloadSelectedAlbumCover() {
        await downloadCoverImage(selectedAlbum);
        await invoke('register_dir', { dir: selectedAlbum.location_on_disk });
    }

    let albumEditDialog;

    function openEditDialog() {
        albumEditDialog.showModal();
    }

    function closeEditDialog() {
        selectedAlbum = null;
        albumEditDialog.close();
    }

    async function getNewCover() {
        const coverPath = await open({
            filters: [{ name: 'Images', extensions: ['jpg', 'png'] }],
            multiple: false
        });

        if (coverPath) {
            albumEditDialog.querySelector('#cover-path').value = coverPath;
            selectedAlbum.cover_path = coverPath;
        }
    }

    async function updateAlbum() {
        let formData = new FormData(albumEditDialog.querySelector('form'));

        await invokeWithToast('update_metadata_album', {
            locationOnDisk: selectedAlbum.location_on_disk,
            coverPath: formData.get('cover-path'),
            title: formData.get('title'),
            artist: formData.get('artist'),
            year: Number(formData.get('year')),
            genre: formData.get('genre'),
        });

        albumEditDialog.close();
        await refreshLibrary();
    }
</script>

<PopoutWindow bind:dialog={albumEditDialog} title={selectedAlbum ? selectedAlbum.title : ""} onClose={closeEditDialog}>
    {#if selectedAlbum}
        <form class="item-edit-form">
            <fieldset>
                <label for="cover-path"><AlbumCover path={selectedAlbum.cover_path}/></label>
                <input on:click={getNewCover} hidden type="text" id="cover-path" name="cover-path">
                <p><strong>WARNING: Tag editing has not been thoroughly tested! Please let me know if you run into issues.</strong></p>
            </fieldset>

            <fieldset>
                <label for="title">
                    <span>Title</span>
                    <input type="text" id="title" name="title" value={selectedAlbum.title}>
                </label>
        
                <label for="artist">
                    <span>Artist</span>
                    <input type="text" id="artist" name="artist" value={selectedAlbum.artist}>
                </label>
        
                <label for="year">
                    <span>Year</span>
                    <input type="number" id="year" name="year" value={selectedAlbum.year}>
                </label>
        
                <label for="genre">
                    <span>Genre</span>
                    <input type="text" id="genre" name="genre" value={selectedAlbum.genre}>
                </label>
            </fieldset>
            
            <fieldset class="footer">
                <button type="button" on:click={updateAlbum}>Save</button>
            </fieldset>
        </form>
    {/if}
</PopoutWindow>
<section bind:this={albumSelector} class="album-selector">
    {#if albums}
        <ul>
            {#each albums as album}
                <li class="album">
                    <Album 
                        on:click={(e) => displayAlbumDetails(e, album)} 
                        on:dblclick={() => playAlbum(album)}
                        on:contextmenu={(e) => showAlbumContextMenu(e, album)}
                        {album}/>
                </li>
            {/each}
        </ul>
    {:else}
        <p>No albums found</p>
    {/if}
    <SongSelector bind:this={songSelector} {activeAlbum} {songList} />
</section>
<ContextMenu bind:this={albumContextMenu}>
    <Item on:click={playSelectedAlbumNext}>Play Next</Item>
    <Item on:click={addSelectedToQueue}>Add to Queue</Item>
    <Item>Shuffle Play</Item>
    <Divider />
    <Item on:click={openEditDialog}>Edit</Item>
    <Item on:click={downloadSelectedAlbumCover}>Download Cover Image</Item>
    <Item on:click={removeSelectedAlbum}>Remove</Item>
    <Divider />
    <Item>Open File Location</Item>
</ContextMenu>

<style>
    .album-selector {
        overflow-x: hidden;
        scroll-behavior: smooth;
        background: var(--clr-gray-1);
        padding: 0.5rem;

        & > ul {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
            gap: 1rem;
        }
    }
</style>