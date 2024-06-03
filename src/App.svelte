<script context="module">
    import { appConfigDir } from '@tauri-apps/api/path';

    console.log(`APP CONFIG: ${await appConfigDir()}`);
</script>

<script>
    import { open } from '@tauri-apps/api/dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import Toasts from './lib/comp/Toasts.svelte';
    import Albums from './lib/windows/Albums.svelte';
    import PlayerControls from './lib/windows/AudioControls.svelte';
    import SongQueue from './lib/windows/SongQueue.svelte';
    import TrackInfo from './lib/windows/TrackInfo.svelte';
    import { onMount, setContext } from 'svelte';
    import { stopPlayback } from './lib/stores/audioPlayer';
    import WindowStack from './lib/comp/WindowStack.svelte';
    import ArtistPage from './lib/windows/ArtistPage.svelte';
    import Artists from './lib/windows/Artists.svelte';
    import { refreshLibrary } from './lib/stores/songLibrary';
    import WindowGroup from './lib/comp/WindowGroup.svelte';
    import { setActiveTab } from './lib/stores/windowManager';
    import TagEditor from './lib/comp/TagEditor.svelte';
    import { invokeWithToast } from './lib/utils';

    let loadingSongs = false;
    let totalSongs = 0;
    let songsRegistered = 0;

    async function openFile() {
        const directory = await open({ directory: true, multiple: false });

        if (directory) {
            loadingSongs = true;
            await invokeWithToast('register_dir', { dir: directory.toString() });
            loadingSongs = false;
            refreshLibrary();
        }
    }

    onMount(async () => {
        setActiveTab('main', 'Albums');
        refreshLibrary();
        stopPlayback();

        await listen('total_songs', (event) => {
            totalSongs = event.payload.message;
        });

        await listen('songs_registered', (event) => {
            songsRegistered = event.payload.message;
        });
    })
</script>

<Toasts />
<TagEditor />

<header id="menu-bar">
    {#if loadingSongs}
        <p>Loading songs: {songsRegistered}/{totalSongs}</p>
    {:else}
        <button on:click={openFile}>Open file</button>
        <button on:click={refreshLibrary}>Refresh library</button>
    {/if}
</header>
<main>
    <Artists />
    <WindowGroup name="main">
        <Albums />
        <ArtistPage />
    </WindowGroup>
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
        grid-template-columns: 16rem 1fr 20rem;
        background: var(--clr-gray-1);

        & > section {
            position: relative;
            margin: 0;
        }
    }
</style>