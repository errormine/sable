<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { setQueue, addToQueue, attemptPlayNext, currentSong } from '../stores/audioPlayer';
    import { loadSongs, openAlbum, refreshLibrary, songList } from '../stores/songLibrary';
    import Album from '../comp/Album.svelte';
    import SongSelector from '../comp/SongSelector.svelte';
    import { downloadCoverImage } from '../stores/lastfmAPI';
    import { openEditDialogFromAlbum, selectedAlbum } from '../stores/tagEditor';

    export let albumList;

    $: albumList, clearSelectedAlbum();

    function clearSelectedAlbum() {
        $openAlbum = null;
        $selectedAlbum = null;
        $songList = [];
    }

    let albumSelector;
    let songSelector;

    async function displayAlbumDetails(e, album) {
        let target = e.currentTarget;
        if ($openAlbum != album) {
            $songList = await loadSongs(album);
            $openAlbum = album;
            
            // Show song selector
            let albumListItem = target.parentNode;
            albumListItem.appendChild(songSelector.domNode);
            songSelector.updateSize(albumListItem);
        } else {
            // Ignore double clicks
            if (e.detail > 1) return;
            $openAlbum = null;
            $songList = [];
        }
    }

    async function playAlbum(album) {
        setQueue(await loadSongs(album), -1);
        attemptPlayNext();
    }

    let albumContextMenu;
    
    function showAlbumContextMenu(e, album) {
        albumContextMenu.show(e);
        $selectedAlbum = album;
    }

    async function removeSelectedAlbum() {
        await invoke('remove_album', { album: $selectedAlbum.title, artist: $selectedAlbum.artist });
        await refreshLibrary();
    }

    async function playSelectedAlbumNext() {
        // TODO: Make this just add the album after the current song
        setQueue(await loadSongs($selectedAlbum));
        if ($currentSong.title == '') {
            attemptPlayNext();
        }
    }

    async function addSelectedToQueue() {
        addToQueue(await loadSongs($selectedAlbum));
    }

    async function downloadSelectedAlbumCover() {
        await downloadCoverImage(selectedAlbum);
        await invoke('register_dir', { dir: $selectedAlbum.location_on_disk });
    }
</script>

<section bind:this={albumSelector} class="album-selector">
    {#if albumList}
        <ul>
            {#each albumList as album}
                <li class="album">
                    <Album {album}
                        on:click={(e) => displayAlbumDetails(e, album)} 
                        on:dblclick={() => playAlbum(album)}
                        on:contextmenu={(e) => showAlbumContextMenu(e, album)}
                            />
                </li>
            {/each}
        </ul>
    {:else}
        <p>No albums found</p>
    {/if}
    <SongSelector bind:this={songSelector} />
</section>
<ContextMenu bind:this={albumContextMenu}>
    <Item on:click={playSelectedAlbumNext}>Play Next</Item>
    <Item on:click={addSelectedToQueue}>Add to Queue</Item>
    <Item>Shuffle Play</Item>
    <Divider />
    <Item on:click={openEditDialogFromAlbum}>Edit</Item>
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