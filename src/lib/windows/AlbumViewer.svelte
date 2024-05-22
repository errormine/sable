<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    let albums;
    let songList;
    let selectedAlbum;

    export async function refreshLibrary() {
        await invoke('get_albums').then(albumsJSON => {
            albums = JSON.parse(albumsJSON);
            console.log(albums);
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

    async function loadSongs(album, artist) {
        songList = null;
        await invoke('get_songs_by_album', { album: album, artist: artist }).then(songsJSON => {
            songList = JSON.parse(songsJSON);
            console.log(songList);
        });
    }
</script>

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
    <section class="song-list">
        {#if songList && selectedAlbum != null}
            <ul>
                {#each songList as song}
                    <li>
                        <button on:click={() => dispatch("playSong", song)}>
                            <p class="song-title">{song.title}</p>
                        </button>
                    </li>
                {/each}
            </ul>
        {/if}
    </section>
</section>

<style>
    .album-viewer {
        width: 100%;
        max-height: calc(100vh - 4rem);
        overflow-y: scroll;
        overflow-x: hidden;
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
        overflow-x: hidden;
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