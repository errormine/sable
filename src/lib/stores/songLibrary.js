import { invoke } from "@tauri-apps/api/core";
import { get, writable } from "svelte/store";

export const albums = writable([]);
export const artists = writable([]);

export async function loadAlbums(artist) {
    return await invoke('get_albums_by_artist', { artist: artist })
        .then(albumsJSON => {
            return JSON.parse(albumsJSON);
        });
}

export async function loadSongs(album) {
    return await invoke('get_songs_by_album', { title: album.title, artist: album.artist })
        .then(songsJSON => {
            return JSON.parse(songsJSON);
        });
}

export async function loadAllSongs() {
    return await invoke('get_all_songs')
        .then(songsJSON => {
            return JSON.parse(songsJSON);
        });
}

export async function refreshLibrary() {
    await invoke('get_all_albums').then(albumsJSON => {
        albums.set(JSON.parse(albumsJSON));
    });

    await invoke('get_all_artists').then(artistsJSON => {
        artists.set(JSON.parse(artistsJSON));
    });
}

export const activeArtist = writable(null);


export const openAlbum = writable(null);
export const songList = writable([]);

export async function refreshSongList(album) {
    songList.set(await loadSongs(album));
}