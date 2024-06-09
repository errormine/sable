<script>
    import Window from "../comp/Window.svelte";
    import { activeArtist, albums, artists, clearActiveArtist, openAlbum } from "../stores/songLibrary";
    import ContextMenu, { Item, Divider } from "svelte-contextmenu";
    import CardListItem from "../comp/CardListItem.svelte";
    import { setActiveTab } from "../stores/windowManager";
    import { lastFm, getArtistInfo } from "../stores/lastfmAPI";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let artistsContextMenu;
    let showAlbums = true;

    function toggleArtistPage(artist) {
        if ($activeArtist === artist) {
            setActiveTab("main", "Albums");
            $activeArtist = null;
            $openAlbum = null;
            return;
        }
        $activeArtist = artist;
        setActiveTab("main", "Artist");
    }

    let artistInfos = {};

    // Artist info loading needs to be refactored so I can update the thurmbnails when you update them from ArtistPage.svelte
    artists.subscribe(async (artists) => {
        if (!artists) return;
        for (let artist of artists) {
            let cachedInfo = localStorage.getItem(artist.name);

            if (cachedInfo) {
                artistInfos[artist.name] = JSON.parse(cachedInfo);
                continue;
            }

            // Wait a bit to avoid rate limiting, results are cached after first request
            await new Promise((resolve) => setTimeout(resolve, 500));
            await getArtistInfo(artist)
                .then((info) => {
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
                <li>
                    <CardListItem 
                        title="All Artists" 
                        subtitle={showAlbums ? $albums.length + " albums" : $artists.reduce((acc, artist) => acc + artist.song_count, 0) + " tracks"}
                        highlighted={!$activeArtist}
                        onClick={clearActiveArtist}
                            >
                        <img src="/assets/placeholder/artist.png" alt="All artists" />
                    </CardListItem>
                </li>
                {#each $artists as artist (artist.name)}
                    <CardListItem 
                        title={artist.name} 
                        subtitle={showAlbums ? artist.album_count + " albums" : artist.song_count + " tracks"}
                        highlighted={artist === $activeArtist}
                        onClick={() => toggleArtistPage(artist)}
                            >
                        {#if artistInfos[artist.name] && artistInfos[artist.name].thumbnail}
                            <img src={convertFileSrc(artistInfos[artist.name].thumbnail)} alt={artist.name} />
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