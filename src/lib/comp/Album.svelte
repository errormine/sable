<script>
    import { path } from '@tauri-apps/api';
    import { convertFileSrc } from '@tauri-apps/api/core';
    import AlbumCover from './AlbumCover.svelte';

    export let album;
</script>

<button class="album" on:click on:dblclick on:contextmenu>
    <section class="cover-wrapper">
        <AlbumCover path={album.cover_path} />
    </section>
    <section class="album-info">
        <p class="title no-wrap" title={album.title}><strong>{album.title}</strong></p>
        <p class="artist no-wrap" title={album.artist}>{album.artist}</p>
    </section>
</button>

<style>
    .album {
        width: 100%;

        & .cover-wrapper {
            aspect-ratio: 1 / 1;
            object-fit: scale-down;
        }
    }

    .album-info {
        padding: 0.5rem 0;
        text-align: center;

        & .title {
            font-size: 1.1rem;
        }
    
        & .artist {
            color: var(--clr-gray-7);
        }
    }
</style>