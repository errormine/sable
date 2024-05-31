<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { setQueue, addToQueue, attemptPlayNext, currentSong } from '../stores/audioPlayer';
    import { loadSongs, refreshLibrary } from '../stores/songLibrary';
    import Album from '../comp/Album.svelte';
    import SongSelector from '../comp/SongSelector.svelte';

    export let albums;

    let activeAlbum;
    let albumSelector;
    let albumEditDialog;
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

<section bind:this={albumSelector} class="album-selector">
    {#if albums}
        <ul>
            {#each albums as album}
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
    <SongSelector bind:this={songSelector} {activeAlbum} {songList} />
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

<style>
    .album-selector {
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