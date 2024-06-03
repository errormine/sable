<script>
    import AlbumCover from "./AlbumCover.svelte";
    import PopoutWindow from "./PopoutWindow.svelte";
    import { closeEditDialog, commitChanges, coverPath, editDialog, getNewCover, selectedSongs } from "../stores/tagEditor";

    export let windowTitle = "Editing tags";
</script>

<PopoutWindow bind:dialog={$editDialog} title={windowTitle} onClose={closeEditDialog}>
    {#if $selectedSongs.length > 0}
        <form class="item-edit-form">
            <fieldset>
                <label for="cover-path"><AlbumCover path={$coverPath}/></label>
                <input on:click={getNewCover} hidden type="text" id="cover-path" name="cover-path" value={$coverPath}>
                <p><strong>WARNING: Tag editing has not been thoroughly tested! Please let me know if you run into issues.</strong></p>
            </fieldset>

            <fieldset>
                <label for="title">
                    <span>Title</span>
                    <input type="text" id="title" name="title" autocomplete="off" placeholder="*" spellcheck="false"
                        value={$selectedSongs.every(song => song.title == $selectedSongs[0].title) ? $selectedSongs[0].title : ""}>
                </label>
        
                <label for="artist">
                    <span>Artist</span>
                    <input type="text" id="artist" name="artist" autocomplete="off" placeholder="*" spellcheck="false"
                        value={$selectedSongs.every(song => song.artist == $selectedSongs[0].artist) ? $selectedSongs[0].artist : ""}>
                </label>
        
                <label for="album-title">
                    <span>Album Title</span>
                    <input type="text" id="album-title" name="album-title" autocomplete="off" placeholder="*" spellcheck="false"
                        value={$selectedSongs.every(song => song.album_title == $selectedSongs[0].album_title) ? $selectedSongs[0].album_title : ""}>
                </label>
        
                <label for="album-artist">
                    <span>Album Artist</span>
                    <input type="text" id="album-artist" name="album-artist" autocomplete="off" placeholder="*" spellcheck="false"
                        value={$selectedSongs.every(song => song.album_artist == $selectedSongs[0].album_artist) ? $selectedSongs[0].album_artist : ""}>
                </label>
        
                <label for="track-number">
                    <span>Track Number</span>
                    <input type="number" id="track-number" name="track-number" autocomplete="off" placeholder="-" spellcheck="false"
                        value={$selectedSongs.every(song => song.track_number == $selectedSongs[0].track_number) ? $selectedSongs[0].track_number : ""}>
                </label>
        
                <label for="disc-number">
                    <span>Disc Number</span>
                    <input type="number" id="disc-number" name="disc-number" autocomplete="off" placeholder="-" spellcheck="false"
                        value={$selectedSongs.every(song => song.disc_number == $selectedSongs[0].disc_number) ? $selectedSongs[0].disc_number : ""}> 
                </label>
        
                <label for="year">
                    <span>Year</span>
                    <input type="number" id="year" name="year" autocomplete="off" placeholder="-" spellcheck="false"
                        value={$selectedSongs.every(song => song.year == $selectedSongs[0].year) ? $selectedSongs[0].year : ""}>
                </label>
        
                <label for="genre">
                    <span>Genre</span>
                    <input type="text" id="genre" name="genre" autocomplete="off" placeholder="*" spellcheck="false"
                        value={$selectedSongs.every(song => song.genre == $selectedSongs[0].genre) ? $selectedSongs[0].genre : ""}>
                </label>
            </fieldset>
            
            <fieldset class="footer">
                <button type="button" on:click={commitChanges}>Save</button>
            </fieldset>
        </form>
    {/if}
</PopoutWindow>