<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { currentSong } from "../stores/audioPlayer";
    import Window from "../comp/Window.svelte";
</script>

<Window title="Track Information">
    <section class="track-info">
        <p>{$currentSong.title}</p>
        <p>{$currentSong.artist}</p>
        <img src={convertFileSrc($currentSong.cover_path)} alt="">
    </section>
</Window>

<style>
    .track-info {
        background: var(--clr-gray-1);
        height: min-content;
    }
</style>