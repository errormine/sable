<svelte:options accessors />
<script>
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
    import { addToast } from '../stores/notifications';
    import IonIosClose from 'virtual:icons/ion/ios-close';
    import { sec2time } from '../utils';
    import { addToQueue, currentSong, insertIntoQueue, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';
    import { activeAlbum, refreshSongList, songList } from '../stores/songLibrary';
    import PopoutWindow from './PopoutWindow.svelte';
    import AlbumCover from './AlbumCover.svelte';
    import IconButton from './IconButton.svelte';

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

        await listen('register_file_finished', (event) => {
            refreshSongList();
            addToast(JSON.parse(event.payload.message));
        });
    });

    let songEditDialog;
    let songContextMenu;
    let selectedSong;
    let editingSong;

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
        editingSong = selectedSong;

        songEditDialog.showModal();
    }

    async function updateSong() {
        let formData = new FormData(songEditDialog.querySelector('form'));

        await invoke('update_metadata_song', {
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
        }).then(async (result) => {
            if (result != "success") {
                addToast({ message: result, type: "error", timeout: 10000 });
                return;
            }
            await invoke('register_file', { filePath: selectedSong.file_path });
        });

        songEditDialog.close();
    }
</script>

<PopoutWindow bind:dialog={songEditDialog} title={editingSong ? editingSong.title : ""}>
    {#if editingSong}
        <form class="item-edit-form">
            <fieldset>
                <label for="cover-path"><AlbumCover path={editingSong.cover_path}/></label>
                <input hidden type="file" id="cover-path" name="cover-path">
            </fieldset>

            <fieldset>
                <label for="title">
                    <span>Title</span>
                    <input type="text" id="title" name="title" value={editingSong.title}>
                </label>
        
                <label for="artist">
                    <span>Artist</span>
                    <input type="text" id="artist" name="artist" value={editingSong.artist}>
                </label>
        
                <label for="album">
                    <span>Album Title</span>
                    <input type="text" id="album-title" name="album-title" value={editingSong.album_title}>
                </label>
        
                <label for="album-artist">
                    <span>Album Artist</span>
                    <input type="text" id="album-artist" name="album-artist" value={editingSong.album_artist}>
                </label>
        
                <label for="track-number">
                    <span>Track Number</span>
                    <input type="number" id="track-number" name="track-number" value={editingSong.track_number}>
                </label>
        
                <label for="disc-number">
                    <span>Disc Number</span>
                    <input type="number" id="disc-number" name="disc-number" value={editingSong.disc_number}>
                </label>
        
                <label for="year">
                    <span>Year</span>
                    <input type="number" id="year" name="year" value={editingSong.year}>
                </label>
        
                <label for="genre">
                    <span>Genre</span>
                    <input type="text" id="genre" name="genre" value={editingSong.genre}>
                </label>
            </fieldset>
            
            <fieldset class="footer">
                <button type="button" on:click={updateSong}>Save</button>
            </fieldset>
        </form>
    {/if}
</PopoutWindow>
<section bind:this={domNode} class="album-info" class:hidden={$activeAlbum == null}>
    <section class="album-info-wrapper">
        {#if $songList && $activeAlbum != null}
            <AlbumCover path={$activeAlbum.cover_path} />
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
    
        &:hover {
            background: var(--clr-gray-5);
        }

        &.active {
            background: var(--clr-gray-0);
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