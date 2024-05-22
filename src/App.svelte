<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { emit, listen } from '@tauri-apps/api/event';
    import AlbumViewer from './lib/windows/AlbumViewer.svelte';
    import PlayerControls from './lib/windows/PlayerControls.svelte';

    let albumViewer;
    let controls;

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
                    albumViewer.refreshLibrary();
                    return;
                }
                songsRegistered = event.payload.message;
            });
        }
    }

    async function playSong(e) {
        controls.play(e.detail.file_path, e.detail.duration);
    }
</script>

<main>
    <section id="menu-bar">
        {#if loadingSongs}
            <p>Loading songs: {songsRegistered}/{totalSongs}</p>
        {:else}
            <button on:click={openFile}>Open file</button>
            <button on:click={() => albumViewer.refreshLibrary()}>Refresh library</button>
        {/if}
    </section>

    <AlbumViewer bind:this={albumViewer} on:playSong={playSong} />
    <PlayerControls bind:this={controls}/>
</main>

<style>
    #menu-bar {
        height: 1.5rem;
    }
</style>