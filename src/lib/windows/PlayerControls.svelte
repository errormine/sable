<script>
    import { sec2time } from '../utils';
    import { onMount } from 'svelte';
    import IonIosPlay from 'virtual:icons/ion/ios-play';
    import IonIosPause from 'virtual:icons/ion/ios-pause';
    import IonIosSkipBackward from 'virtual:icons/ion/ios-skipbackward';
    import IonIosSkipforward from 'virtual:icons/ion/ios-skipforward';
    import IonIosShuffle from 'virtual:icons/ion/ios-shuffle';
    import IonIosRepeat from 'virtual:icons/ion/ios-repeat';
    import IonVolumeHigh from 'virtual:icons/ion/volume-high';
    import IconButton from '../comp/IconButton.svelte';
    import Slider from '../comp/Slider.svelte';
    import { attemptPlayNext, currentSong, isPlaying, songProgress, stopPlayback, togglePlayback } from '../stores/audioPlayer';
    import { invoke } from '@tauri-apps/api/tauri';

    let progressBar;
    let userSeeking = false;
    
    songProgress.subscribe(async (value) => {
        if (userSeeking || !progressBar) return;
        progressBar.setValue(value);
        if (value >= $currentSong.duration) {
            stopPlayback();
            attemptPlayNext();
        }
    });

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
        <Slider bind:this={progressBar} name="progress-bar" id="progress-bar" min={0} max={$currentSong.duration || 999} />
        <p>{sec2time($currentSong.duration)}</p>
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
        <Slider width="6rem" color="var(--clr-gray-9)" name="volume-slider" id="volume-slider" min={0} max={100} value={80} />
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
        gap: 1rem;
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