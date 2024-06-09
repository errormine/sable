<script>
    import IonIosClose from "virtual:icons/ion/ios-close";
    import AlbumSelector from "../comp/AlbumSelector.svelte";
    import IconButton from "../comp/IconButton.svelte";
    import Window from "../comp/Window.svelte";
    import ContextMenu, { Item } from "svelte-contextmenu";
    import { getArtistImage, getArtistImages, getArtistInfo, lastFm, setNewPortrait } from "../stores/lastfmAPI";
    import { activeArtist, artistInfos, clearActiveArtist, loadAlbums, openAlbum } from "../stores/songLibrary";
    import PopoutWindow from "../comp/PopoutWindow.svelte";
    import { convertFileSrc } from "@tauri-apps/api/core";
    
    let albums;
    let artistInfo;
    let activeTab = 'albums';

    artistInfos.subscribe((infos) => {
        if (!$activeArtist) return;
        artistInfo = infos[$activeArtist.name];
    });

    activeArtist.subscribe(async (artist) => {
        if (!artist) {
            return;
        }

        albums = await loadAlbums(artist.name);
        artistInfo = $artistInfos[artist.name];
    });

    let portraitDialog;
    let portraitContextMenu;
    let portraitSources = [];
    let selectedNewPortrait;

    async function showPortraitDialog() {
        let artist = await getArtistInfo($activeArtist);
        portraitSources = await getArtistImages(artist);
        portraitDialog.showModal();
    }

    async function closePortraitDialog() {
        portraitSources = null;
        portraitDialog.close();
    }

    async function updatePortrait(src) {
        setNewPortrait($activeArtist.name, src)
            .then(async () => {
                $artistInfos[$activeArtist.name] = await getArtistInfo($activeArtist);
                portraitDialog.close();
            });
    }
</script>

<PopoutWindow bind:dialog={portraitDialog} title="Select Portrait" onClose={closePortraitDialog}>
    {#if portraitSources != null && $activeArtist != null}
        <form class="portrait-selector">
            <section class="portraits-wrapper">
                <section class="portraits">
                    {#each portraitSources as src}
                        <button type="button" on:click={() => selectedNewPortrait = src} class:active={selectedNewPortrait == src}>
                            <img {src} alt={$activeArtist.name} />
                        </button>
                    {/each}
                </section>
            </section>
            <fieldset class="footer">
                <button type="button" on:click={() => updatePortrait(selectedNewPortrait)}>Set Portrait</button>
            </fieldset>
        </form>
    {:else}
        <p>No portraits found</p>
    {/if}
</PopoutWindow>
<Window title="Artist">
    <section class="artist">
        {#if $activeArtist}
            <header class="hero">
                <ContextMenu bind:this={portraitContextMenu}>
                    <Item on:click={showPortraitDialog}>Download New Portrait</Item>
                    <Item>Open on Last.fm</Item>
                </ContextMenu>
                {#if artistInfo != null && artistInfo.thumbnail}
                    <img src={convertFileSrc(artistInfo.thumbnail)} alt={$activeArtist.name} on:contextmenu={(e) => portraitContextMenu.show(e)}>
                {:else}
                    <img src="/assets/placeholder/artist.png" alt={$activeArtist.name} on:contextmenu={(e) => portraitContextMenu.show(e)}>
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
                <IconButton on:click={clearActiveArtist} >
                    <IonIosClose />
                </IconButton>
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
    .portrait-selector {
        max-width: 50vw;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;

        & .portraits-wrapper {
            overflow-x: auto;
            padding: 0.5rem;
        }
        
        & .portraits {
            white-space: nowrap;
            width: max-content;
            display: flex;
            gap: 0.5rem;
        }

        & img {
            display: block;
        }

        & button {
            overflow: hidden;
            border-radius: 0.5rem;
            display: inline-block;
        }

        & button.active {
            background: var(--clr-gray-3);
            color: var(--clr-gray-9);
            outline: 1px solid var(--clr-gray-7);
        }
    }
    
    .artist {
        display: grid;
        padding: 1rem;
        height: 100%;
        grid-template-rows: min-content 1fr;
    }

    .hero {
        display: grid;
        grid-template-columns: 12rem 1fr 2rem;
        gap: 1rem;
        margin-bottom: 1rem;

        & > img {
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