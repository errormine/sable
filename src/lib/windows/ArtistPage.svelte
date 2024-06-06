<script>
    import AlbumSelector from "../comp/AlbumSelector.svelte";
    import Window from "../comp/Window.svelte";
    import { getArtistImage, getArtistInfo, lastFm } from "../stores/lastfmAPI";
    import { activeArtist, loadAlbums } from "../stores/songLibrary";
    
    let albums;
    let artistInfo;
    let activeTab = 'albums';

    activeArtist.subscribe(async (artist) => {
        if (!artist) {
            return;
        }

        albums = await loadAlbums(artist.name);
        artistInfo = await getArtistInfo(artist);
    });
</script>

<Window title="Artist">
    <section class="artist">
        {#if $activeArtist}
            <header class="hero">
                {#if artistInfo && artistInfo.thumbnail}
                    <img src={artistInfo.thumbnail} alt={$activeArtist.name}>
                {:else}
                    <img src="/assets/placeholder/artist.png" alt={$activeArtist.name}>
                {/if}
                <section class="artist-info">
                    <h1 class="artist-name">{$activeArtist.name}</h1>
                    {#if artistInfo != null}
                        <p class="tags">
                            {#each artistInfo.tags.tag as tag}
                                <a href={tag.url} target="_blank" class="tag">
                                    {tag.name}
                                </a>
                            {/each}
                        </p>
                    {/if}
                    <nav>
                        <ul>
                            <li class:active={activeTab == 'albums'}>
                                <button on:click={() => activeTab = 'albums'}>Albums</button>
                            </li>
                            <li class:active={activeTab == 'about'}>
                                <button on:click={() => activeTab = 'about'}>About</button>
                            </li>
                            <li class:active={activeTab == 'lastfm'}>
                                <button on:click={() => activeTab = 'lastfm'}>Last.fm Stats</button>
                            </li>
                        </ul>
                    </nav>
                </section>
            </header>
            <section class="content">
                {#if activeTab == 'albums' && albums}
                    <AlbumSelector albumList={albums} />
                {/if}
                {#if activeTab == 'about'}
                    <section class="biography">
                        <h2>Biography</h2>
                        {#if artistInfo && artistInfo.bio}
                            <p>{@html artistInfo.bio.content}</p>
                        {:else}
                            <p>No biography available</p>
                        {/if}
                    </section>
                {/if}
            </section>
        {:else}
            <p>No artist selected</p>
        {/if}
    </section>
</Window>

<style>
    .artist {
        display: grid;
        padding: 1rem;
        height: 100%;
        grid-template-rows: min-content 1fr;
    }

    .hero {
        display: grid;
        grid-template-columns: 12rem 1fr;
        gap: 1rem;
        margin-bottom: 1rem;

        & img {
            border-radius: 50%;
        }

        & .artist-info {
            display: flex;
            flex-direction: column;
            justify-content: center;
            gap: 0.5rem;
        }

        & .artist-name {
            font-size: 3rem;
            line-height: 1;
            margin: 0;
        }

        & .tags {
            display: flex;
            gap: 0.5rem;
        }

        & .tag {
            padding: 0.25rem 0.5rem;
            background: var(--clr-gray-3);
            border-radius: 0.5rem;
            color: var(--clr-gray-9);
            text-decoration: none;
            text-transform: uppercase;
            line-height: 1;
        }
    }

    nav {
        & ul {
            display: flex;
            gap: 1.5rem;
        }
        
        & button {
            padding: 0.5rem 1rem;
            font-size: 1.25rem;
            border-radius: 0.5rem;
            color: var(--clr-gray-7);
        }

        & .active button {
            background: var(--clr-gray-3);
            color: var(--clr-gray-9);
        }
    }

    .biography {
        margin: 0.5rem 0;
        padding: 1rem;
        background: var(--clr-gray-3);
        border-radius: 0.5rem;
        white-space: pre-wrap;
    }

    .content {
        overflow-y: scroll;
    }
</style>