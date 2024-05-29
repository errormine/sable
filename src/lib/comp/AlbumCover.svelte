<script>
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    export let path = "";
    export let rounded = true;
</script>

<img src={path == "" ? "/assets/placeholder/cover.png" : convertFileSrc(path)} alt="" width="128" height="128" loading="lazy" class:rounded={rounded}>

<style>
    img {
        display: block;
        width: 100%;
    }

    img.rounded {
        border-radius: 0.25rem;
    }
</style>