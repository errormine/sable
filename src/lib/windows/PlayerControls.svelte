<script context="module">
    import { invoke } from '@tauri-apps/api/tauri';
    import { get, writable } from 'svelte/store';
    import IonIosPlay from 'virtual:icons/ion/ios-play';
    import IonIosPause from 'virtual:icons/ion/ios-pause';
    import IonIosSkipBackward from 'virtual:icons/ion/ios-skipbackward';
    import IonIosSkipforward from 'virtual:icons/ion/ios-skipforward';
    import { currentlyPlaying } from './TrackInfo.svelte';
    import { attemptPlayNext } from './SongQueue.svelte';

    let songProgress = writable(0);
    let isPlaying = writable(false);

    let intervalIndex;
    let progressBar;
    let userSeeking = false;

    export async function play(song) {
        if (!song.file_path) return;

        if (get(currentlyPlaying).file_path != song.file_path) {
            await invoke('play', { filePath: song.file_path });
            songProgress.set(0);
            currentlyPlaying.set(song);
        }

        beginPlayBack();
    }

    async function togglePlayback() {
        get(isPlaying) ? pausePlayback() : beginPlayBack();
    }

    async function beginPlayBack() {
        clearInterval(intervalIndex);
        await invoke('resume');
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

    export async function stopPlayback() {
        await invoke('stop');
        clearInterval(intervalIndex);
        isPlaying.set(false);
    }

    songProgress.subscribe(async (value) => {
        if (userSeeking) return;
        if (progressBar) {
            progressBar.value = value;
            if (value >= get(currentlyPlaying).duration) {
                clearInterval(intervalIndex);
                isPlaying.set(false);
                attemptPlayNext();
            }
            updateProgressBarStyle();
        }
    });

    function updateProgressBarStyle() {
        progressBar.style.setProperty('--value', progressBar.value);
        progressBar.style.setProperty('--min', progressBar.min);
        progressBar.style.setProperty('--max', progressBar.max);
    }
</script>

<script>
    import { sec2time } from '../utils';
    import { onMount } from 'svelte';
    import IconButton from '../comp/IconButton.svelte';

    onMount(() => {
        progressBar.addEventListener('input', (event) => {
            userSeeking = true;
            updateProgressBarStyle();
        });

        progressBar.addEventListener('mouseup', async (event) => {
            let newTime = event.target.value;
            console.log(`Seeking to : ${newTime}`);
            await invoke('seek', { position: newTime });
            songProgress.set(Number(newTime));
            userSeeking = false;
        });
    });
</script>

<footer>
    <section id="main-controls">
        <IconButton>
            <IonIosSkipBackward />
        </IconButton>
        <IconButton on:click={togglePlayback}>
            {#if $isPlaying}
                <IonIosPause/>
            {:else}
                <IonIosPlay/>
            {/if}
        </IconButton>
        <IconButton>
            <IonIosSkipforward/>
        </IconButton>
    </section>

    <section id="progress-controls">
        <label for="progress-bar">{sec2time($songProgress)}</label>
        <input bind:this={progressBar} class="styled-slider slider-progress" type="range" name="progress-bar" id="progress-bar" min="0" max={$currentlyPlaying.duration} />
        <p>{sec2time($currentlyPlaying.duration)}</p>
    </section>

    <section id="secondary-controls">
        <IconButton>
            <!-- <ShuffleOutline size="xl" /> -->
        </IconButton>
        <IconButton>
            <!-- <ArrowsRepeatOutline size="xl" /> -->
        </IconButton>
        <IconButton>
            <!-- <VolumeUpSolid size="xl" /> -->
        </IconButton>
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
        height: var(--controls-height);
        background: var(--clr-gray-1);
    }

    footer > section {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    label[for="progress-bar"] {
        pointer-events: none;
    }

    #progress-bar {
        width: 50vw;
        margin: 0 1rem;
    }
</style>