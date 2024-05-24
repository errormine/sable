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
    import { onMount } from 'svelte';
    import { sec2time } from '../utils';
    import { currentSong, play, setQueue } from '../stores/audioPlayer';
    import Window from '../comp/Window.svelte';

    let albumViewer;
    let songList = [];
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
        });
    }

    function playSongAndQueue(song) {
        play(song);
        setQueue(songList, song.track_number);
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

<Window title="Albums">
    <section bind:this={albumViewer} class="album-viewer">
        {#if $albums}
            <ul>
                {#each $albums as album}
                    <li class="album">
                        <button on:click={(e) => selectAlbum(e.currentTarget, album)}>
                            <section class="cover-wrapper">
                                <img src={convertFileSrc(album.cover_path)} alt="" width="128" height="128" loading="lazy">
                            </section>
                            <p class="title no-wrap" title={album.title}><strong>{album.title}</strong></p>
                            <p class="no-wrap" title={album.artist}>{album.artist}</p>
                        </button>
                    </li>
                {/each}
            </ul>
        {:else}
            <p>No albums found</p>
        {/if}
        <section bind:this={songSelector} class="song-selector" class:hidden={activeAlbum == null}>
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
                                <!-- so long!!!! -->
                                <button title={song.title} 
                                    class:active={$currentSong.title == song.title && $currentSong.artist == song.artist}
                                    on:click={() => playSongAndQueue(song)}>
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
    </section>
</Window>

<style>
    .album-viewer {
        max-height: var(--main-window-height);
        overflow-y: scroll;
        overflow-x: hidden;
        scroll-behavior: smooth;
        padding: 0 0.5rem;
        background: var(--clr-gray-1);
    }

    .album-viewer > ul {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
        gap: 1rem;
    }

    .album button {
        width: 100%;
    }

    .cover-wrapper {
        aspect-ratio: 1 / 1;
        object-fit: scale-down;
        margin-bottom: 0.5rem;
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
        margin-top: 1rem;
        gap: 1rem;
        background: linear-gradient(white, var(--clr-gray-3));
        border-top: 1px solid var(--clr-gray-5);
        border-bottom: 1px solid var(--clr-gray-5);
    }

    .song-selector .song-list {
        column-count: auto;
        column-width: 22vw;
        column-gap: 3rem;
    }
    
    .song-selector .song button {
        display: grid;
        grid-template-columns: 3ch 1fr auto;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        gap: 1.5rem;
        transition: all 200ms;
    }
    
    .song-selector .song button:hover {
        opacity: 0.75;
    }

    :global(.song-selector .song button.active) {
        outline: 1px solid var(--clr-gray-5);
        background: linear-gradient(white, var(--clr-gray-3));
    }
    
    .song-selector .song * {
        display: inline;
        text-align: left;
    }
    
    .song .song-title {
        font-weight: normal;
    }

    .song .duration {
        float: right;
    }
</style>