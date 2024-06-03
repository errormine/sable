import { get, writable } from 'svelte/store';
import { open } from "@tauri-apps/api/dialog";
import { invokeWithToast } from '../utils';
import { invoke } from '@tauri-apps/api/tauri';
import { addToast } from './notifications';
import { loadSongs, refreshLibrary, refreshSongList } from './songLibrary';

export const editDialog = writable(null);
export const selectedAlbum = writable(null);
export const selectedSongs = writable([]);
export const coverPath = writable(null);

export function openEditDialog() {
    if (!get(selectedAlbum) || get(selectedSongs).length == 0) {
        return;
    }

    coverPath.set(get(selectedAlbum).cover_path);
    get(editDialog).showModal();
}

export async function openEditDialogFromAlbum() {
    let songs = await loadSongs(get(selectedAlbum));
    selectedSongs.set(songs);
    openEditDialog();
}

export function closeEditDialog() {
    get(editDialog).close();
}

export async function getNewCover() {
    const newPath = await open({
        filters: [{ name: 'Images', extensions: ['jpg', 'png'] }],
        multiple: false
    });

    if (newPath) {
        get(editDialog).querySelector('#cover-path').value = newPath;
        coverPath.set(newPath);
    }
}

export const editInProgress = writable(false);
export const editProgress = writable(0);
export const editTotal = writable(0);

export async function commitChanges() {
    if (!get(selectedAlbum) || !get(selectedSongs)) {
        return;
    }

    editInProgress.set(true);
    editProgress.set(0);
    editTotal.set(get(selectedSongs).length);
    let formData = new FormData(get(editDialog).querySelector('form'));
    let results = [];

    for (let song of get(selectedSongs)) {
        await invoke('update_metadata_song', {
            locationOnDisk: get(selectedAlbum).location_on_disk,
            filePath: song.file_path,
            coverPath: formData.get('cover-path') || song.cover_path,
            title: formData.get('title') || song.title,
            artist: formData.get('artist') || song.artist,
            albumTitle: formData.get('album-title') || song.album_title,
            albumArtist: formData.get('album-artist') || song.album_artist,
            trackNumber: Number(formData.get('track-number')) || song.track_number,
            discNumber: Number(formData.get('disc-number')) || song.disc_number,
            year: Number(formData.get('year')) || song.year,
            genre: formData.get('genre') || song.genre
        })
        .then(result => {
            results.push(result);
        })
        .catch(error => {
            console.error(error);
        });
        editProgress.update(n => n + 1);
    }

    editInProgress.set(false);
    closeEditDialog();
    addToast({
        message: `Updated ${results.length} song${results.length > 1 ? 's' : ''}`,
        type: 'success',
        timeout: 3000,
        dismissable: true
    });
    refreshLibrary();
}