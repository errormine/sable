<script>
    import { convertFileSrc } from "@tauri-apps/api/core";
    import Window from "../comp/Window.svelte";
    import { loadAllSongs } from "../stores/songLibrary";
</script>

<Window title="Songs">
    <section class="songs">
        {#await loadAllSongs()}
            <p>Loading...</p>
        {:then songs}
            <ul class="song-grid">
                {#each songs as song}
                    <li>
                        <img src={convertFileSrc(song.cover_path)} alt="">
                    </li>
                {/each}
            </ul>
        {:catch error}
            <p>{error.message}</p>
        {/await}
    </section>
</Window>

<style>
    .songs {
        max-height: inherit;
        overflow-y: scroll;
        padding: 1rem;
    }

    .song-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(32px, 1fr));
        gap: 0.5rem;
        list-style: none;
        padding: 0;
        margin: 0;
    }

    img {
        border-radius: 0.25rem;
    }
</style>