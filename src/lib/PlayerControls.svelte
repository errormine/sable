<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { ArrowsRepeatOutline, BackwardStepSolid, ForwardStepSolid, PauseSolid, PlaySolid, ShuffleOutline, StopSolid, VolumeUpSolid } from 'flowbite-svelte-icons';

    let progressBar;
    let currentSong = '';
    let songProgress = 0;
    let songDuration = 0;
    let isPlaying = false;

    let progressInterval;

    function sec2time(seconds) {
        let minutes = Math.floor(seconds / 60);
        let secs = Math.floor(seconds % 60);

        return `${minutes}:${secs < 10 ? '0' + secs : secs}`;
    };

    export async function play(filePath, duration) {
        if (!filePath) return;

        if (filePath != currentSong) {
            await invoke('play', { filePath });
            songProgress = 0;
            songDuration = duration;
            currentSong = filePath;
        }

        beginPlayBack();
    }

    async function togglePlayback() {
        isPlaying ? pausePlayback() : beginPlayBack();
    }

    async function beginPlayBack() {
        clearInterval(progressInterval);
        await invoke('resume');
        progressInterval = setInterval(async () => {
            songProgress += 1;
        }, 1000);
        isPlaying = true;
    }

    async function pausePlayback() {
        await invoke('pause');
        clearInterval(progressInterval);
        isPlaying = false;
    }

    async function skipBackward() {
        await invoke('skip_backward');
    }

    async function skipForward() {
        await invoke('skip_forward');
    }

    $: progressBar?.addEventListener('input', async () => {
        // TODO: Implement seeking
    });
</script>

<footer>
    <section id="main-controls">
        <button on:click={skipBackward}>
            <BackwardStepSolid />
        </button>
        <button on:click={togglePlayback}>
            {#if isPlaying}
                <PauseSolid />
            {:else}
                <PlaySolid />
            {/if}
        </button>
        <button on:click={skipForward}>
            <ForwardStepSolid />
        </button>
    </section>

    <section id="progress-controls">
        <input bind:this={progressBar} type="range" name="progress-bar" id="progress-bar" min="0" max={songDuration}>
        <label for="progress-bar">{sec2time(songProgress)} / {sec2time(songDuration)}</label>
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
    }
</style>