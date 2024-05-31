<script>
    import { invoke } from "@tauri-apps/api";
    import Window from "../comp/Window.svelte";
    import { artists } from "../stores/songLibrary";
    import CardListItem from "../comp/CardListItem.svelte";
    import { setActiveTab } from "../stores/windowManager";

    let activeArtist;

    function toggleArtistPage(artist) {
        if (activeArtist === artist) {
            setActiveTab("main", "Albums");
            activeArtist = null;
            return;
        }
        activeArtist = artist;
        setActiveTab("main", "Artist");
    }
</script>

<Window title="Artists">
    <section class="artists">
        {#if $artists}
            <ul class="artist-list">
                {#each $artists as artist}
                    <CardListItem 
                        title={artist.name} 
                        subtitle={artist.album_count + " albums"}
                        highlighted={artist === activeArtist}
                        onClick={() => toggleArtistPage(artist)}
                            >
                        <img src="/assets/placeholder/artist.png" alt={artist.name} />
                    </CardListItem>
                {/each}
            </ul>
        {:else}
            <p>No artists found</p>
        {/if}
    </section>
</Window>

<style>
    img {
        border-radius: 50%;
    }

    .artists {
        padding: 0 0.25rem;
        overflow-y: auto;
    }

    .artist-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>