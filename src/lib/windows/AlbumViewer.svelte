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
    import IonIosClose from 'virtual:icons/ion/ios-close';
    import { setQueue, addToQueue, attemptPlayNext, currentSong } from '../stores/audioPlayer';
    import { activeAlbum, loadSongs, refreshSongList, songList } from '../stores/songLibrary';
    import Window from '../comp/Window.svelte';
    import Album from '../comp/Album.svelte';
    import SongSelector from '../comp/SongSelector.svelte';

    const albumViewer = writable(null);
    setContext('albumViewer', albumViewer);

    let albumEditDialog;
    let songSelector;

    async function displayAlbumDetails(e, album) {
        let target = e.currentTarget;
        if ($activeAlbum != album) {
            $activeAlbum = album;
            refreshSongList();
            
            // Show song selector
            let albumListItem = target.parentNode;
            albumListItem.appendChild(songSelector.domNode);
            songSelector.updateSize(albumListItem);
            $albumViewer.scrollTo(0, target.offsetTop - 40);
        } else {
            // Ignore double clicks
            if (e.detail > 1) return;
            $activeAlbum = null;
        }
    }

    async function playAlbum(album) {
        setQueue(await loadSongs(album));
        attemptPlayNext();
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

    async function playSelectedAlbumNext() {
        setQueue(await loadSongs(selectedAlbum));
        if ($currentSong.title == '') {
            attemptPlayNext();
        }
    }

    async function addSelectedToQueue() {
        addToQueue(await loadSongs(selectedAlbum));
    }
</script>

<Window title="Albums">
    <section bind:this={$albumViewer} class="album-viewer">
        {#if $albums}
            <ul>
                {#each $albums as album}
                    <li class="album">
                        <Album 
                            on:click={(e) => displayAlbumDetails(e, album)} 
                            on:dblclick={() => playAlbum(album)}
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
        <Item on:click={playSelectedAlbumNext}>Play Next</Item>
        <Item on:click={addSelectedToQueue}>Add to Queue</Item>
        <Item>Shuffle Play</Item>
        <Divider />
        <Item on:click={() => albumEditDialog.showModal()}>Edit</Item>
        <Item on:click={removeSelectedAlbum}>Remove</Item>
        <Divider />
        <Item>Open File Location</Item>
    </ContextMenu>
</Window>

<style>
    .album-viewer {
        overflow-y: auto;
        scroll-behavior: smooth;
        padding: 0.5rem;
        background: var(--clr-gray-1);

        & > ul {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
            gap: 1rem;
        }
    }
</style>