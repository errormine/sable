<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import AlbumViewer, { refreshLibrary } from './lib/windows/AlbumViewer.svelte';
    import PlayerControls from './lib/windows/PlayerControls.svelte';
    import SongQueue from './lib/windows/SongQueue.svelte';

    let controls;

    let loadingSongs = false;
    let totalSongs = 0;
    let songsRegistered = 0;

    let songQueue;

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
                    refreshLibrary();
                    return;
                }
                songsRegistered = event.payload.message;
            });
        }
    }

    refreshLibrary();
</script>

<header id="menu-bar">
    {#if loadingSongs}
        <p>Loading songs: {songsRegistered}/{totalSongs}</p>
    {:else}
        <button on:click={openFile}>Open file</button>
        <button on:click={refreshLibrary}>Refresh library</button>
    {/if}
</header>
<main>
    <section id="left-window">

    </section>
    <section id="middle-window">
        <AlbumViewer />
    </section>
    <section id="right-window">
        <SongQueue bind:this={songQueue} />
    </section>
</main>
<PlayerControls bind:this={controls}/>

<style>
    #menu-bar {
        height: 1.5rem;
    }

    main {
        display: grid;
        grid-template-columns: 15rem 1fr 20rem;
    }
</style>