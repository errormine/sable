<script>
    import AlbumSelector from "../comp/AlbumSelector.svelte";
import Window from "../comp/Window.svelte";
    import { activeArtist, loadAlbums } from "../stores/songLibrary";

    let albums;

    activeArtist.subscribe(async (artist) => {
        if (!artist) {
            return;
        }

        albums = await loadAlbums(artist.name);
    });
</script>

<Window title="Artist">
    <section class="artist">
        {#if $activeArtist}
            <h1>{$activeArtist.name}</h1>
            {#if albums}
                <AlbumSelector {albums} />
            {/if}
        {:else}
            <p>No artist selected</p>
        {/if}
    </section>
</Window>