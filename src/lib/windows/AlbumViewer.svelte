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
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { createEventDispatcher, onMount } from 'svelte';
    import { songQueue, currentSongIndex } from '../../lib/stores/queue.js';
    import { play } from './PlayerControls.svelte';
    import { currentlyPlaying } from './TrackInfo.svelte';

    let albumViewer;
    let songList;
    let songSelector;
    let activeAlbum;

    async function selectAlbum(target, album) {
        if (activeAlbum != album) {
            loadSongs(album.title, album.artist);
            activeAlbum = album;

            // Show song selector
            target.parentNode.appendChild(songSelector);
            resizeSongSelector(target.parentNode);
            albumViewer.scrollTo(0, songSelector.parentNode.offsetTop - 25);
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

    async function playSongAndQueue(song) {
        play(song.file_path, song.duration);
        currentlyPlaying.set(song);
        songQueue.set(songList);
        currentSongIndex.set(song.track_number);
    }

    function resizeSongSelector(offsetNode) {
        // Hack to keep the song selector the correct size
        let difference = albumViewer.offsetLeft;
        songSelector.style.left = -offsetNode.offsetLeft + difference + 'px';
        songSelector.style.width = albumViewer.clientWidth + 'px';
    }

    onMount(() => {
        addEventListener('resize', () => {
            // TODO: fix errors when parent node doesn't exist
            resizeSongSelector(songSelector.parentNode);
        });
    });
</script>

<section bind:this={albumViewer} class="album-viewer">
    {#if $albums}
        <ul>
            {#each $albums as album}
                <li class="album">
                    <button on:click={(e) => selectAlbum(e.currentTarget, album)}>
                        <section class="cover-wrapper">
                            <img src={convertFileSrc(album.cover_path)} alt="" width="128" height="128" loading="lazy">
                        </section>
                        <p class="title no-wrap"><strong>{album.title}</strong></p>
                        <p class="no-wrap">{album.artist}</p>
                    </button>
                </li>
            {/each}
        </ul>
    {:else}
        <p>No albums found</p>
    {/if}
    <section bind:this={songSelector} class="song-selector">
        {#if songList && activeAlbum != null}
            <img class="large-album-cover" src={convertFileSrc(activeAlbum.cover_path)} alt="">
            <section class="album-info">
                <header class="mb-05">
                    <h2>{activeAlbum.title}</h2>
                    <p class="subtitle">{activeAlbum.artist}</p>
                </header>
                <ol class="song-list">
                    {#each songList as song}
                        <li class="song">
                            <button on:click={() => playSongAndQueue(song)}>
                                <p class="song-title no-wrap"><span>{song.track_number}</span>{song.title}</p>
                            </button>
                        </li>
                    {/each}
                </ol>
            </section>
        {/if}
    </section>
</section>

<style>
    .album-viewer {
        max-height: calc(100vh - 4rem);
        overflow-y: scroll;
        overflow-x: hidden;
        scroll-behavior: smooth;
        padding: 0.25rem;
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

    .song-selector {
        position: relative;
        display: grid;
        grid-template-columns: 16rem 1fr;
        padding: 1rem;
        gap: 1rem;
    }

    .song-selector .song-list {
        column-count: auto;
        column-width: 20vw;
    }

    .song-selector .song {
        padding: 0.1rem 0.5rem;
        border-radius: 0.25rem;
    }

    .song-selector .song:hover {
        outline: 1px solid black;
    }
    
    .song-selector .song-title {
        text-align: left;
        font-weight: normal;
    }

    .song-title span {
        display: inline-block;
        margin-right: 1.5rem;
        width: 2ch;
    }
</style>