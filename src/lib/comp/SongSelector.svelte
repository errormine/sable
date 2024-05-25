<svelte:options accessors />
<script>
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { sec2time } from '../utils';
    import { currentSong, play, setQueue } from '../stores/audioPlayer';
    import { getContext, onMount } from 'svelte';

    export let domNode = null;

    const activeAlbum = getContext('activeAlbum');
    const songList = getContext('songList');

    let owner = null;
    
    export function updateSize(offsetNode) {
        // Hack to keep the song selector the correct size
        let difference = owner.offsetLeft;
        domNode.style.left = -offsetNode.offsetLeft + difference + 'px';
        domNode.style.width = owner.clientWidth + 'px';
    }

    function playSongAndQueue(song) {
        play(song);
        setQueue($songList, song.track_number);
    }
    
    onMount(() => {
        owner = domNode.parentNode;

        addEventListener('resize', () => {
            // TODO: fix errors when parent node doesn't exist
            updateSize(domNode.parentNode);
        });
    });
</script>

<section bind:this={domNode} class="song-selector" class:hidden={$activeAlbum == null}>
    {#if $songList && $activeAlbum != null}
        <img class="large-album-cover" src={convertFileSrc($activeAlbum.cover_path)} alt="">
        <section class="album-info">
            <header class="mb-05">
                <h2>{$activeAlbum.title}</h2>
                <p class="subtitle">{$activeAlbum.artist}</p>
            </header>
            <ol class="song-list">
                {#each $songList as song}
                    <li class="song">
                        <!-- so long!!!! -->
                        <button title={song.title} 
                            class:active={$currentSong.title == song.title && $currentSong.artist == song.artist}
                            on:click={() => playSongAndQueue(song)}>
                            <span class="track-number">{song.track_number}</span>
                            <p class="song-title no-wrap">{song.title}</p>
                            <span class="duration">{sec2time(song.duration)}</span>
                        </button>
                    </li>
                {/each}
            </ol>
        </section>
    {/if}
</section>

<style>
    .song-selector {
        position: relative;
        display: grid;
        grid-template-columns: 16rem 1fr;
        padding: 1rem;
        margin-top: 1rem;
        gap: 1rem;
        background: linear-gradient(white, var(--clr-gray-3));
        border-top: 1px solid var(--clr-gray-5);
        border-bottom: 1px solid var(--clr-gray-5);
    }

    .song-selector .song-list {
        column-count: auto;
        column-width: 22vw;
        column-gap: 3rem;
    }
    
    .song-selector .song button {
        display: grid;
        grid-template-columns: 3ch 1fr auto;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        gap: 1.5rem;
        transition: all 200ms;
        width: 100%;
    }
    
    .song-selector .song button:hover {
        opacity: 0.75;
    }

    :global(.song-selector .song button.active) {
        outline: 1px solid var(--clr-gray-5);
        background: linear-gradient(white, var(--clr-gray-3));
    }
    
    .song-selector .song * {
        display: inline;
        text-align: left;
    }
    
    .song .song-title {
        font-weight: normal;
    }

    .song .duration {
        float: right;
    }
</style>