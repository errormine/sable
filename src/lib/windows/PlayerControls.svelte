<script context="module">
    import { invoke } from '@tauri-apps/api/tauri';
    import { get, writable } from 'svelte/store';
    import { ArrowsRepeatOutline, BackwardStepSolid, ForwardStepSolid, PauseSolid, PlaySolid, ShuffleOutline, StopSolid, VolumeUpSolid } from 'flowbite-svelte-icons';
    import { attemptPlayNext } from './SongQueue.svelte';

    let currentSong = '';
    let songProgress = writable(0);
    let songDuration = writable(0);
    let isPlaying = writable(false);

    let intervalIndex;

    export async function play(filePath, duration) {
        if (!filePath) return;

        if (filePath != currentSong) {
            await invoke('play', { filePath });
            songProgress.set(0);
            songDuration.set(duration);
            currentSong = filePath;
        }

        beginPlayBack();
    }

    async function togglePlayback() {
        isPlaying ? pausePlayback() : beginPlayBack();
    }

    async function beginPlayBack() {
        clearInterval(intervalIndex);
        invoke('resume');
        intervalIndex = setInterval(async () => {
            songProgress.update((n) => n + 1);
        }, 1000);
        isPlaying.set(true);
    }

    async function pausePlayback() {
        await invoke('pause');
        clearInterval(intervalIndex);
        isPlaying.set(false);
    }

    songProgress.subscribe(async (value) => {
        if (value >= get(songDuration)) {
            clearInterval(intervalIndex);
            isPlaying.set(false);
            attemptPlayNext();
        }
    });
</script>

<script>
    import { sec2time } from '../utils';
</script>

<footer>
    <section id="main-controls">
        <button>
            <BackwardStepSolid />
        </button>
        <button on:click={togglePlayback}>
            {#if $isPlaying}
                <PauseSolid />
            {:else}
                <PlaySolid />
            {/if}
        </button>
        <button>
            <ForwardStepSolid />
        </button>
    </section>

    <section id="progress-controls">
        <input type="range" name="progress-bar" id="progress-bar" min="0" max={$songDuration}>
        <label for="progress-bar">{sec2time($songProgress)} / {sec2time($songDuration)}</label>
    </section>

    <section id="secondary-controls">
        <button>
            <ShuffleOutline />
        </button>
        <button>
            <ArrowsRepeatOutline />
        </button>
        <button>
            <VolumeUpSolid />
        </button>
        <input type="range" min="0" max="100" step="1" />
    </section>
</footer>

<style>
    footer {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        width: 100%;
        align-content: center;
        justify-content: space-between;
        padding: 0.5rem;
        background-color: white;
        height: 2.5rem;
    }
</style>