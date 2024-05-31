import { invoke } from "@tauri-apps/api/tauri";
import { get, writable } from "svelte/store";

export const activeAlbum = writable(null);
export const songList = writable(null);

export async function loadSongs(album) {
    return await invoke('get_songs_by_album', { title: album.title, artist: album.artist })
        .then(songsJSON => {
            return JSON.parse(songsJSON);
        });
}

export async function refreshSongList() {
    let album = get(activeAlbum);
    if (album == null) return;
    songList.set(await loadSongs(album));
}

export const albums = writable([]);
export const artists = writable([]);

export async function refreshLibrary() {
    await invoke('get_all_albums').then(albumsJSON => {
        albums.set(JSON.parse(albumsJSON));
    });

    await invoke('get_all_artists').then(artistsJSON => {
        artists.set(JSON.parse(artistsJSON));
        console.log(get(artists));
    });
}