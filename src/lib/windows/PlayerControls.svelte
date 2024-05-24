<script context="module">
    import { invoke } from '@tauri-apps/api/tauri';
    import { get, writable } from 'svelte/store';
    import IonIosPlay from 'virtual:icons/ion/ios-play';
    import IonIosPause from 'virtual:icons/ion/ios-pause';
    import IonIosSkipBackward from 'virtual:icons/ion/ios-skipbackward';
    import IonIosSkipforward from 'virtual:icons/ion/ios-skipforward';
    import IonIosShuffle from 'virtual:icons/ion/ios-shuffle';
    import IonIosRepeat from 'virtual:icons/ion/ios-repeat';
    import IonVolumeHigh from 'virtual:icons/ion/volume-high';
    import { currentlyPlaying } from './TrackInfo.svelte';
    import { attemptPlayNext } from './SongQueue.svelte';

    let songProgress = writable(0);
    let isPlaying = writable(false);

    let intervalIndex;
    let userSeeking = false;

    export async function play(song) {
        if (!song.file_path) return;

        if (get(currentlyPlaying) != song) {
            console.log('Playing new song: ', song.title);
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
        if (get(isPlaying)) return;
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

</script>

<script>
    import { sec2time } from '../utils';
    import { onMount } from 'svelte';
    import IconButton from '../comp/IconButton.svelte';
    import Slider from '../comp/Slider.svelte';

    let progressBar;

    onMount(() => {
        if (!progressBar) return;
        
        progressBar.input.addEventListener('input', () => {
            userSeeking = true;
        });
        
        progressBar.input.addEventListener('mouseup', async (event) => {
            // @ts-ignore
            let newTime = event.target.value;
            if (!newTime) return;
            console.log(`Seeking to : ${newTime}`);
            await invoke('seek', { position: newTime });
            songProgress.set(Number(newTime));
            userSeeking = false;
        });

        songProgress.subscribe(async (value) => {
            if (userSeeking) return;
            progressBar.setValue(value);
            if (value >= $currentlyPlaying.duration) {
                clearInterval(intervalIndex);
                isPlaying.set(false);
                attemptPlayNext();
            }
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
        <Slider bind:this={progressBar} name="progress-bar" id="progress-bar" min={0} max={$currentlyPlaying.duration || 999} />
        <p>{sec2time($currentlyPlaying.duration)}</p>
    </section>

    <section id="secondary-controls">
        <IconButton>
            <IonIosShuffle/>
        </IconButton>
        <IconButton>
            <IonIosRepeat/>
        </IconButton>
        <IconButton>
            <IonVolumeHigh/>
        </IconButton>
        <Slider color="var(--clr-gray-9)" name="volume-slider" id="volume-slider" min={0} max={100} value={80} />
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
        padding: 0.5rem 1.5rem;
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

    :global(#progress-bar) {
        width: 50vw;
        margin: 0 1rem;
    }
</style>