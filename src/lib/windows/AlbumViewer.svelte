<script context="module">
    import { writable } from 'svelte/store';

    let albums = writable([]);

    export async function refreshLibrary() {
        await invoke('get_albums').then(albumsJSON => {
            albums.set(JSON.parse(albumsJSON));
        });
    }
</script>

<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { getContext, onMount, setContext } from 'svelte';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { setQueue, addToQueue, attemptPlayNext } from '../stores/audioPlayer';
    import Window from '../comp/Window.svelte';
    import Album from '../comp/Album.svelte';
    import SongSelector from '../comp/SongSelector.svelte';
    import { set } from 'tauri-settings';

    const albumViewer = writable(null);
    setContext('albumViewer', albumViewer);
    let songSelector;

    const activeAlbum = writable(null);
    setContext('activeAlbum', activeAlbum);
    const songList = writable(null);
    setContext('songList', songList);

    async function selectAlbum(target, album) {
        if ($activeAlbum != album) {
            $songList = await loadSongs(album.title, album.artist);
            $activeAlbum = album;
            
            // Show song selector
            let albumListItem = target.parentNode;
            albumListItem.appendChild(songSelector.domNode);
            songSelector.updateSize(albumListItem);
            $albumViewer.scrollTo(0, target.offsetTop - 40);
        } else {
            $activeAlbum = null;
        }
    }

    let albumContextMenu;
    let selectedAlbum;
    
    function showAlbumMenu(e, album) {
        albumContextMenu.show(e);
        selectedAlbum = album;
    }

    async function removeSelectedAlbum() {
        await invoke('remove_album', { album: selectedAlbum.title, artist: selectedAlbum.artist });
        await refreshLibrary();
    }

    async function playSelectedAlbum() {
        setQueue(await loadSongs(selectedAlbum.title, selectedAlbum.artist));
        attemptPlayNext();
    }

    async function addSelectedToQueue() {
        addToQueue(await loadSongs(selectedAlbum.title, selectedAlbum.artist));
    }

    async function loadSongs(album, artist) {
        return await invoke('get_songs_by_album', { album: album, artist: artist })
            .then(songsJSON => {
                return JSON.parse(songsJSON);
            });
    }
</script>

<Window title="Albums">
    <section bind:this={$albumViewer} class="album-viewer">
        {#if $albums}
            <ul>
                {#each $albums as album}
                    <li class="album">
                        <Album 
                            on:click={(e) => selectAlbum(e.currentTarget, album)} 
                            on:contextmenu={(e) => showAlbumMenu(e, album)}
                            {album}/>
                    </li>
                {/each}
            </ul>
        {:else}
            <p>No albums found</p>
        {/if}
        <SongSelector bind:this={songSelector} />
    </section>
    <ContextMenu bind:this={albumContextMenu}>
        <Item on:click={playSelectedAlbum}>Play Next</Item>
        <Item on:click={addSelectedToQueue}>Add to Queue</Item>
        <Item>Shuffle Play</Item>
        <Divider />
        <Item>Edit</Item>
        <Item on:click={removeSelectedAlbum}>Remove</Item>
        <Divider />
        <Item>Open File Location</Item>
    </ContextMenu>
</Window>

<style>
    .album-viewer {
        overflow-y: scroll;
        overflow-x: hidden;
        scroll-behavior: smooth;
        padding: 0.5rem;
        background: var(--clr-gray-1);
    }

    .album-viewer > ul {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
        gap: 1rem;
    }
</style>