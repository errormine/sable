import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";
import { getRecord, insertRecord } from "./stronghold";
import LastFM from "tauri-lastfm";

export const lastFm = new LastFM({
    apiKey: import.meta.env.VITE_LASTFM_API_KEY,
    apiSecret: import.meta.env.VITE_LASTFM_API_SECRET,
    verbose: true
});

export function getAuthUrl(token) {
    return lastFm.auth.getAuthUrl(token);
}

export function getToken() {
    return lastFm.auth.getToken();
}

export function getSession() {
    return getRecord("lastfm_session")
        .then(session => {
            if (session) {
                return session;
            }

            return lastFm.auth.getSession();
        })
        .catch(err => {
            console.error(err);
        });
}

async function download(url, dest) {
    const response = await fetch(url);

    await invoke("write_file", { path: dest, contents: response.json() });
}

async function getArtistImageLastFm(artist) {
    if (!artist || !artist.url) return null;
    
    let lastFmImage = fetch(artist.url)
        .then(res => {
            return res.text();
        }).then(data => {
            let parser = new DOMParser();
            let doc = parser.parseFromString(data, 'text/html');
            let images = doc.querySelectorAll('.image-list-item');
            if (images.length === 0) {
                return null;
            }
            let img = images[0].querySelector('img');

            if (!img || !img.src) {
                return null;
            }

            return img.src;
        });

    if (lastFmImage) {
        return lastFmImage;
    }

    return getArtistImageDeezer(artist);
}

async function getArtistImageDeezer(artist) {
    return fetch(`https://api.deezer.com/search/artist?q="${artist.name}"&limit=1&strict=on`)
        .then(res => {
            return res.json();
        })
        .then(data => {
            return data.data[0].picture_xl;
        });
}

export async function getArtistImage(name) {
    let artistInfo = await lastFm.artist.getInfo({ artist: name });

    return artistInfo.thumbnail;

}

export async function getAlbumId(title, artist) {
    return fetch(`https://api.deezer.com/search/album?q=artist:"${artist}" album:"${title}"&limit=1`)
        .then(res => res.json())
        .then(data => {
            return data.data[0].id;
        });
}

export async function getAlbumDeezer(id) {
    return fetch(`https://api.deezer.com/album/${id}`)
        .then(res => res.json())
        .then(data => {
            return data;
        });
}

export async function getAlbumImage(title, artist) {
    let albumId = await getAlbumId(title, artist);
    let album = await getAlbumDeezer(albumId);

    return album.cover_xl;
}

export async function downloadCoverImage(album) {
    let albumImage = await getAlbumImage(album.title, album.artist);
    let dest = album.location_on_disk + "/Cover.jpg";

    await download(albumImage, dest);
}