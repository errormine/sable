import { invoke } from "@tauri-apps/api/core";
import { get, writable } from "svelte/store";
import { setActiveTab } from "./windowManager";

export const albums = writable([]);
export const artists = writable([]);

export async function loadAlbums(name) {
    return await invoke('get_albums_by_artist', { artist: name })
        .then(albumsJSON => {
            return JSON.parse(albumsJSON);
        });
}

export async function loadAllAlbums() {
    return await invoke('get_all_albums')
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

export async function clearActiveArtist() {
    activeArtist.set(null);
    openAlbum.set(null);
    setActiveTab('main', 'Albums');
}