<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { emit, listen } from '@tauri-apps/api/event';
    import PlayerControls from './lib/PlayerControls.svelte';

    let controls;
    let albums;
    let selectedAlbum;
    let songList;

    let loadingSongs = false;
    let totalSongs = 0;
    let songsRegistered = 0;

    async function openFile() {
        const result = await open({ directory: true, multiple: false });

        if (result) {
            loadingSongs = true;
            invoke('register_songs', { dir: result.toString() });
            await listen('total_songs', (event) => {
                totalSongs = event.payload.message;
            });
            await listen('songs_registered', (event) => {
                if (event.payload.message == "done") {
                    loadingSongs = false;
                    return;
                }
                songsRegistered = event.payload.message;
            });
        }
    }

    async function refreshLibrary() {
        await invoke('get_albums').then(albumsJSON => {
            albums = JSON.parse(albumsJSON);
            console.log(albums);
        });
    }

    async function loadSongs(album, artist) {
        songList = null;
        await invoke('get_songs_by_album', { album: album, artist: artist }).then(songsJSON => {
            songList = JSON.parse(songsJSON);
            console.log(songList);
        });
    }

    async function selectAlbum(target, album) {
        if (selectedAlbum != target) {
            loadSongs(album.title, album.artist);
            selectedAlbum = target;
        } else {
            selectedAlbum = null;
        }
    }

    refreshLibrary();
</script>

<main>
    {#if loadingSongs}
        <p>Loading songs: {songsRegistered}/{totalSongs}</p>
    {:else}
        <button on:click={openFile}>Open file</button>
        <button on:click={refreshLibrary}>Refresh library</button>
    {/if}

    <section class="album-viewer">
        {#if albums}
            <ul>
                {#each albums as album}
                    <li class="album">
                        <button on:click={(e) => selectAlbum(e.target, album)}>
                            <img src={convertFileSrc(album.cover_path)} alt="">
                            <p class="title"><strong>{album.title}</strong></p>
                            <p>{album.artist}</p>
                        </button>
                    </li>
                {/each}
            </ul>
        {:else}
            <p>No albums found</p>
        {/if}
    </section>
    <section class="song-list">
        {#if songList && selectedAlbum != null}
            <ul>
                {#each songList as song}
                    <li>
                        <button on:click={() => controls.play(song.file_path, song.duration)}>
                            <p class="song-title">{song.title}</p>
                        </button>
                    </li>
                {/each}
            </ul>
        {/if}
    </section>
    <PlayerControls bind:this={controls}/>
</main>

<style>
    .album-viewer {
        width: 100%;
    }

    .album-viewer > ul {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
        gap: 1rem;
    }

    .album-viewer .album {
        border-radius: 0.5rem;
    }

    .album button {
        width: 100%;
    }

    .album-viewer img {
        width: 100%;
        border-radius: 0.25rem;
    }

    .album-viewer .title,
    .song-list .song-title {
        overflow: hidden;
        text-overflow: ellipsis;
        text-wrap: nowrap;
    }

    .song-list ul {
        padding: 1rem;
        list-style-type: decimal;
        column-count: auto;
        column-width: 18rem;
    }

    .song-list .song-title {
        font-weight: normal;
    }
</style>