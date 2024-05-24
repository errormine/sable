<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { currentSong } from "../stores/audioPlayer";
</script>

<section class="track-info">
    <p>Track Information</p>
    <section>
        <p>{$currentSong.title}</p>
        <p>{$currentSong.artist}</p>
    </section>
    <img src={convertFileSrc($currentSong.cover_path)} alt="">
</section>

<style>
    .track-info {
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }
</style>