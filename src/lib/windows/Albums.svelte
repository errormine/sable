<script>
    import Window from '../comp/Window.svelte';
    import AlbumSelector from '../comp/AlbumSelector.svelte';
    import { albums } from '../stores/songLibrary';
</script>

<Window title="Albums">
    <AlbumSelector albums={$albums} />
</Window>