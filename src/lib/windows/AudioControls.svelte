<script>
    import { sec2time } from '../utils';
    import { onMount } from 'svelte';
    import { addToast } from '../stores/notifications';
    import IonIosPlay from 'virtual:icons/ion/ios-play';
    import IonIosPause from 'virtual:icons/ion/ios-pause';
    import IonIosSkipBackward from 'virtual:icons/ion/ios-skipbackward';
    import IonIosSkipforward from 'virtual:icons/ion/ios-skipforward';
    import IonIosShuffle from 'virtual:icons/ion/ios-shuffle';
    import IonIosRepeat from 'virtual:icons/ion/ios-repeat';
    import IonVolumeHigh from 'virtual:icons/ion/volume-high';
    import IconButton from '../comp/IconButton.svelte';
    import Slider from '../comp/Slider.svelte';
    import { attemptPlayNext, attemptPlayPrevious, currentSong, isPlaying, songProgress, startedPlayingAt, stopPlayback, togglePlayback } from '../stores/audioPlayer';
    import { invoke } from '@tauri-apps/api/core';
    import { getSession, lastFm, lastFmConnected } from '../stores/lastfmAPI';

    let progressBar;
    let volumeSlider;
    let userSeeking = false;
    let totalDurationListened = 0;
    let timeOfScrobble = 0;
    let canScrobbleAgain = true;
    
    songProgress.subscribe(async (value) => {
        if (userSeeking || !progressBar) return;
        progressBar.setValue(value);
        if (value >= $currentSong.duration) {
            stopPlayback();
            attemptPlayNext();
        }

        // Keep track of how much of the song has been listened to
        // we should only scrobble if the user has listened to more than half of the song
        if (!$lastFmConnected) return;
        totalDurationListened += 1;
        if (value <= timeOfScrobble) {
            canScrobbleAgain = true;
            totalDurationListened = 0;
            timeOfScrobble = 0;
        }
        if (totalDurationListened >= $currentSong.duration / 2 && canScrobbleAgain) {
            canScrobbleAgain = false;
            timeOfScrobble = value;
            let session = await getSession();
            console.log(`Scrobbling song: ${$currentSong.title}`);
            await lastFm.track.scrobble({
                    // @ts-ignore
                    artist: $currentSong.artist,
                    track: $currentSong.title,
                    album: $currentSong.album_title,
                    albumArtist: $currentSong.album_artist,
                    trackNumber: $currentSong.track_number,
                    duration: $currentSong.duration,
                    timestamp: $startedPlayingAt.toString()
            }, session.key)
            .then(result => console.log(result));
        }
    });
    
    onMount(async () => {
        if (!progressBar) return;
        
        progressBar.input.addEventListener('input', () => {
            userSeeking = true;
        });
        
        progressBar.input.addEventListener('mouseup', async (event) => {
            let newTime = event.target.value;
            if (!newTime) return;
            console.log(`Seeking to : ${newTime}`);
            await invoke('seek', { position: newTime })
                .then(result => {
                    if (result == "success") {
                        songProgress.set(Number(newTime));
                    } else {
                        addToast({
                            message: result,
                            type: 'error',
                            dismissable: true,
                            timeout: 5000,
                        });
                    }
                });
            userSeeking = false;
        });

        volumeSlider.input.addEventListener('input', async (event) => {
            let newVolume = event.target.value;
            if (!newVolume) return;
            await invoke('set_volume', { volume: Number(newVolume) });
        });

        volumeSlider.input.addEventListener('change', async (event) => {
            let newVolume = event.target.value;
            if (!newVolume) return;
        });
    });
</script>

<footer>
    <section id="main-controls">
        <IconButton on:click={attemptPlayPrevious}>
            <IonIosSkipBackward />
        </IconButton>
        <IconButton on:click={togglePlayback}>
            {#if $isPlaying}
                <IonIosPause/>
            {:else}
                <IonIosPlay/>
            {/if}
        </IconButton>
        <IconButton on:click={attemptPlayNext}>
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
        <Slider bind:this={volumeSlider} width="6rem" color="var(--clr-gray-9)" name="volume-slider" id="volume-slider" min={0} max={100} value={80} />
    </section>
</footer>

<style>
    footer {
        display: flex;
        width: 100%;
        align-content: center;
        justify-content: space-between;
        padding: 0.5rem 1.5rem;
        height: var(--controls-height);
        background: var(--clr-gray-1);
        gap: 1rem;
        border-top: 1px solid var(--clr-gray-3);

        & > section {
            display: flex;
            align-items: center;
            gap: 1rem;
        }
    }

    label[for="progress-bar"] {
        pointer-events: none;
    }

    :global(#progress-bar) {
        width: 50vw;
        margin: 0 1rem;
    }
</style>