<script>
    import { convertFileSrc } from '@tauri-apps/api/tauri';

    export let album;
</script>

<button on:click on:dblclick on:contextmenu>
    <section class="cover-wrapper">
        <img src={convertFileSrc(album.cover_path)} alt="" width="128" height="128" loading="lazy">
    </section>
    <p class="title no-wrap" title={album.title}><strong>{album.title}</strong></p>
    <p class="no-wrap" title={album.artist}>{album.artist}</p>
</button>

<style>
    button {
        width: 100%;
    }

    .cover-wrapper {
        aspect-ratio: 1 / 1;
        object-fit: scale-down;
        margin-bottom: 0.5rem;
    }
    
    img {
        width: 100%;
        border-radius: 0.25rem;
    }
</style>