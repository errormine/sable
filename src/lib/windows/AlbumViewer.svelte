<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    let albums;
    let songList;
    let songSelector;
    let activeAlbum;

    export async function refreshLibrary() {
        await invoke('get_albums').then(albumsJSON => {
            albums = JSON.parse(albumsJSON);
            console.log(albums);
        });
    }

    async function selectAlbum(target, album) {
        let selection = { _element: target, meta: album };

        if (activeAlbum != selection) {
            loadSongs(album.title, album.artist);
            activeAlbum = selection;

            // Show song selector
            target.parentNode.appendChild(songSelector);
            songSelector.style.left = -target.offsetLeft + 'px';
        } else {
            activeAlbum = null;
        }
    }

    async function loadSongs(album, artist) {
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
                    <button on:click={(e) => selectAlbum(e.currentTarget, album)}>
                        <section class="cover-wrapper">
                            <img src={convertFileSrc(album.cover_path)} alt="" width="128" height="128" loading="lazy">
                        </section>
                        <p class="title"><strong>{album.title}</strong></p>
                        <p>{album.artist}</p>
                    </button>
                </li>
            {/each}
        </ul>
    {:else}
        <p>No albums found</p>
    {/if}
    <section bind:this={songSelector} class="song-selector">
        {#if songList && activeAlbum != null}
            <img class="large-album-cover" src={convertFileSrc(activeAlbum.meta.cover_path)} alt="">
            <section class="album-info">
                <h2>{activeAlbum.meta.title}</h2>
                <ul>
                    {#each songList as song}
                        <li>
                            <button on:click={() => dispatch("playSong", song)}>
                                <p class="song-title"><span>{song.track_number}</span>{song.title}</p>
                            </button>
                        </li>
                    {/each}
                </ul>
            </section>
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

    .album-viewer .cover-wrapper {
        aspect-ratio: 1 / 1;
        object-fit: scale-down;
    }
    
    .album-viewer img {
        width: 100%;
        border-radius: 0.25rem;
    }

    .album-viewer .title,
    .song-selector .song-title {
        overflow-x: hidden;
        text-overflow: ellipsis;
        text-wrap: nowrap;
    }

    .song-selector {
        position: relative;
        width: 100vw;
        display: grid;
        grid-template-columns: 16rem 1fr;
        padding: 1rem;
        gap: 1rem;
    }

    .song-selector ul {
        column-count: auto;
        column-width: 16rem;
        column-gap: 2rem;
    }
    
    .song-selector .song-title {
        text-align: left;
        font-weight: normal;
    }

    .song-title span {
        margin-right: 1rem;
    }
</style>