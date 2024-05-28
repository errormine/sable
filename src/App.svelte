<script context="module">
    import { appConfigDir } from '@tauri-apps/api/path';

    console.log(`APP CONFIG: ${await appConfigDir()}`);
</script>

<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import Toasts from './lib/comp/Toasts.svelte';
    import AlbumViewer, { refreshLibrary } from './lib/windows/AlbumViewer.svelte';
    import PlayerControls from './lib/windows/AudioControls.svelte';
    import SongQueue from './lib/windows/SongQueue.svelte';
    import TrackInfo from './lib/windows/TrackInfo.svelte';
    import { onMount } from 'svelte';
    import { stopPlayback } from './lib/stores/audioPlayer';
    import WindowStack from './lib/comp/WindowStack.svelte';
    import { addToast } from './lib/stores/notifications';

    let loadingSongs = false;
    let totalSongs = 0;
    let songsRegistered = 0;

    async function openFile() {
        const directory = await open({ directory: true, multiple: false });

        if (directory) {
            loadingSongs = true;
            invoke('register_dir', { dir: directory.toString() });
        }
    }

    onMount(async () => {
        refreshLibrary();
        stopPlayback();

        await listen('total_songs', (event) => {
            totalSongs = event.payload.message;
        });

        await listen('songs_registered', (event) => {
            songsRegistered = event.payload.message;
        });

        await listen('register_songs_finished', (event) => {
            loadingSongs = false;
            refreshLibrary();
            addToast(JSON.parse(event.payload.message));
        });
    })
</script>

<Toasts />

<header id="menu-bar">
    {#if loadingSongs}
        <p>Loading songs: {songsRegistered}/{totalSongs}</p>
    {:else}
        <button on:click={openFile}>Open file</button>
        <button on:click={refreshLibrary}>Refresh library</button>
    {/if}
</header>
<main>
    <section id="left">

    </section>
    <AlbumViewer />
    <WindowStack id="right">
        <SongQueue />
        <TrackInfo />
    </WindowStack>
</main>
<PlayerControls />

<style>
    #menu-bar {
        height: var(--menu-bar-height);
    }

    main {
        max-height: var(--main-window-height);
        display: grid;
        grid-template-columns: 0 1fr 20rem;
        background: var(--clr-gray-1);

        & > section {
            position: relative;
            margin: 0;
        }
    }
</style>