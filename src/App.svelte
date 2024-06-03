<script context="module">
    import { appConfigDir } from '@tauri-apps/api/path';

    console.log(`APP CONFIG: ${await appConfigDir()}`);
</script>

<script>
    import { open } from '@tauri-apps/api/dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import Toasts from './lib/comp/Toasts.svelte';
    import Albums from './lib/windows/Albums.svelte';
    import ContextMenu, { Item, Divider } from 'svelte-contextmenu';
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
    import Songs from './lib/windows/Songs.svelte';

    let fileContextMenu;

    let loadingSongs = false;
    let songsTotal = 0;
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
            songsTotal = event.payload.message;
        });

        await listen('songs_registered', (event) => {
            songsRegistered = event.payload.message;
        });
    })
</script>

<Toasts />
<TagEditor />
<ContextMenu bind:this={fileContextMenu}>
    <Item on:click={openFile}>Add Folder...</Item>
    <Item on:click={refreshLibrary}>Refresh Library</Item>
</ContextMenu>

<header class="menubar">
    {#if loadingSongs}
        <p>Registering songs... <progress max={songsTotal} value={songsRegistered}></progress></p>
    {:else}
        <button on:click={(e) => fileContextMenu.show(e)}>File</button>
        <button>Edit</button>
        <button>View</button>
        <button>Help</button>
    {/if}
</header>
<main>
    <Artists />
    <WindowGroup name="main">
        <Albums />
        <ArtistPage />
        <Songs />
    </WindowGroup>
    <WindowStack id="right">
        <SongQueue />
        <TrackInfo />
    </WindowStack>
</main>
<PlayerControls />

<style>
    .menubar {
        height: var(--menu-bar-height);
        display: flex;
        color: var(--clr-gray-7);
        background: var(--clr-gray-2);

        & button {
            padding: 0 0.5rem;
            transition: all 200ms;
        }

        & button:hover {
            color: var(--clr-gray-9);
            background: var(--clr-gray-4);
        }
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