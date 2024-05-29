<script>
    import { currentSong } from "../stores/audioPlayer";
    import Window from "../comp/Window.svelte";
    import AlbumCover from "../comp/AlbumCover.svelte";
</script>

<Window title="Track Information">
    <section class="track-info">
        <header>
            <p>{$currentSong.title}</p>
            <p>{$currentSong.artist}</p>
        </header>
        <AlbumCover path={$currentSong.cover_path} rounded={false}/>
    </section>
</Window>

<style>
    .track-info {
        background: var(--clr-gray-1);
        height: min-content;

        & > header {
            padding: 1rem;
        }
    }
</style>