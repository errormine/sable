<script context="module">
    import { appConfigDir } from '@tauri-apps/api/path';

    console.log(`APP CONFIG: ${await appConfigDir()}`);
</script>

<script>
    import { open } from '@tauri-apps/plugin-dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import Toasts from './lib/comp/Toasts.svelte';
    import Albums from './lib/windows/Albums.svelte';
    import ContextMenu, { Item, Divider, Settings, defaultSettings } from 'svelte-contextmenu';
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
    import { getToken, getSession, getAuthUrl, lastFm, lastFmConnected } from './lib/stores/lastfmAPI';
    import PopoutWindow from './lib/comp/PopoutWindow.svelte';
    import { getRecord } from './lib/stores/stronghold';

    const contextSettings = new Settings();
    contextSettings.Menu.Class.push('context-menu');
    contextSettings.Menu.VisibleClass.push('context-menu-active');
    contextSettings.Item.Class.push('context-item');
    contextSettings.Divider.Class.push('context-divider');

    defaultSettings.set(contextSettings);

    let fileContextMenu;

    let loadingSongs = false;
    let songsTotal = 0;
    let songsRegistered = 0;
    let justRegistered = '';

    async function openFile() {
        const directory = await open({ directory: true, multiple: false });

        if (directory) {
            loadingSongs = true;
            await invokeWithToast('register_dir', { dir: directory.toString() });
            loadingSongs = false;
            refreshLibrary();
        }
    }

    async function authenticateLastFm() {
        let url = await getAuthUrl();
        window.open(url, '_blank');
        $lastFmConnected = true;
    }

    async function printSession() {
        let session = await getSession();
        console.log(session);
    }

    async function lastFmPrint() {
        let token = await getToken();
        let session = await getSession();
        console.log(session);
    }

    onMount(async () => {
        let lastFmName = await getRecord('lastfm_name');
        $lastFmConnected = lastFmName != '';
        setActiveTab('main', 'Albums');
        refreshLibrary();
        stopPlayback();

        await listen('total_songs', (event) => {
            songsTotal = event.payload.message;
        });

        await listen('song_registered', (event) => {
            songsRegistered += 1;
            justRegistered = event.payload.message;
        });
    })
</script>

<Toasts />
<TagEditor />
<ContextMenu bind:this={fileContextMenu}>
    <Item on:click={openFile}>Add Folder...</Item>
    <Item on:click={refreshLibrary}>Refresh Library</Item>
    <Item on:click={authenticateLastFm}>Link Last.fm Account</Item>
    {#if import.meta.env.DEV}
        <Item on:click={printSession}>Print Last.fm Session</Item>
    {/if}
</ContextMenu>

<header class="menubar">
    {#if loadingSongs}
        <section class="menubar-registering no-wrap">
            <p>Registering songs: </p>
            <progress max={songsTotal} value={songsRegistered}></progress>
            <p class="no-wrap">{justRegistered}</p>
        </section>
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

    .menubar-registering {
        display: flex;
        align-items: center;
        gap: 1rem;
        width: 100%;
        padding: 0 1rem;
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