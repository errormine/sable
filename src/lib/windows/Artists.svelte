<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import Window from "../comp/Window.svelte";
    import { activeArtist, artists } from "../stores/songLibrary";
    import ContextMenu, { Item, Divider } from "svelte-contextmenu";
    import CardListItem from "../comp/CardListItem.svelte";
    import { setActiveTab } from "../stores/windowManager";
    import { getArtistImage, getArtistInfo, retrieveFromCache } from "../stores/lastfmAPI";

    let artistsContextMenu;
    let showAlbums = true;

    function toggleArtistPage(artist) {
        if ($activeArtist === artist) {
            setActiveTab("main", "Albums");
            $activeArtist = null;
            return;
        }
        $activeArtist = artist;
        setActiveTab("main", "Artist");
    }

    let artistInfos = {};

    artists.subscribe(async (artists) => {
        for (let artist of artists) {
            let cached = await retrieveFromCache(artist.name);
            
            if (cached) {
                artistInfos[artist.name] = cached;
                continue;
            }

            // Wait a bit to avoid rate limiting, results are cached after first request
            await new Promise((resolve) => setTimeout(resolve, 500));
            await getArtistInfo(artist.name).then((info) => {
                artistInfos[artist.name] = info;
                artistInfos = artistInfos;
            });
        }
    });
</script>

<ContextMenu bind:this={artistsContextMenu}>
    <Item on:click={() => showAlbums = true}>
        {showAlbums ? "✓" : " "} Show albums
    </Item>
    <Item on:click={() => showAlbums = false}>
        {!showAlbums ? "✓" : " "} Show tracks
    </Item>
</ContextMenu>
<Window title="Artists" contextMenu={artistsContextMenu}>
    <section class="artists">
        {#if $artists}
            <ul class="artist-list">
                {#each $artists as artist (artist.name)}
                    <CardListItem 
                        title={artist.name} 
                        subtitle={showAlbums ? artist.album_count + " albums" : artist.song_count + " tracks"}
                        highlighted={artist === $activeArtist}
                        onClick={() => toggleArtistPage(artist)}
                            >
                        {#if artistInfos[artist.name] && artistInfos[artist.name].thumbnail}
                            <img src={artistInfos[artist.name].thumbnail} alt={artist.name} />
                        {:else}
                            <img src="/assets/placeholder/artist.png" alt={artist.name} />
                        {/if}
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