import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";
import { getRecord, insertRecord } from "./stronghold";
import LastFM from "tauri-lastfm";
import { writable } from "svelte/store";
import { appDataDir, sep } from "@tauri-apps/api/path";
import { invokeWithToast } from "../utils";

export const lastFmConnected = writable(false);

export const lastFm = new LastFM({
    apiKey: import.meta.env.VITE_LASTFM_API_KEY,
    apiSecret: import.meta.env.VITE_LASTFM_API_SECRET,
    verbose: import.meta.env.DEV
});

export function getAuthUrl(token) {
    return lastFm.auth.getAuthUrl(token);
}

export function getToken() {
    return lastFm.auth.getToken();
}

export async function getSession() {
    let name = await getRecord("lastfm_name");
    let key = await getRecord("lastfm_key");
    let subscriber = await getRecord("lastfm_subscriber");

    if (!name || !key || !subscriber) {
        return lastFm.auth.getSession()
            .then(session => {
                insertRecord("lastfm_name", session.name);
                insertRecord("lastfm_key", session.key);
                insertRecord("lastfm_subscriber", session.subscriber);

                return session;
            })
            .catch(err => {
                console.error(err);
                return null;
            });
    }

    return { name, key, subscriber };
}

export async function getArtistInfo(artist) {
    let cachedInfo = localStorage.getItem(artist.name);

    if (cachedInfo) {
        return JSON.parse(cachedInfo);
    }

    let artistInfo = await lastFm.artist.getInfo({ artist: artist.name });
    let thumbnailUrl = await getArtistImageLastFm(artistInfo.artist);
    if (thumbnailUrl) {
        let portraitSrc = await downloadPortrait(thumbnailUrl);
        let artistWithPortrait = { ...artistInfo.artist, thumbnail: portraitSrc };
        localStorage.setItem(artist.name, JSON.stringify(artistWithPortrait));
    }

    return JSON.parse(localStorage.getItem(artist.name));
}

export async function downloadPortrait(url) {
    let dest = await appDataDir();
    let seperator = sep();
    dest += `${seperator}portraits`;
    let fileName = url.split("/").pop() + ".jpg";
    
    await invokeWithToast("download", { url, dest, name: fileName });
    
    return dest + `${seperator}${fileName}`;
}

export async function setNewPortrait(name, url) {
    let artist = JSON.parse(localStorage.getItem(name));
    let finalPath = await downloadPortrait(url);
    localStorage.setItem(name, JSON.stringify({ ...artist, thumbnail: finalPath }));
}

export async function getArtistImages(artist) {
    if (!artist || !artist.url) return null;
    console.log(`Fetching images for ${artist.name}`);
    
    return fetch(artist.url)
        .then(res => {
            return res.text();
        }).then(data => {
            let parser = new DOMParser();
            let doc = parser.parseFromString(data, 'text/html');
            let images = doc.querySelectorAll('.image-list-item');
            if (images.length === 0) {
                return null;
            }

            let sources = [];

            images.forEach(img => {
                let source = img.querySelector('img').src;

                if (!sources.includes(source)) {
                    sources.push(source);
                }
            });

            if (!sources || sources.length === 0) {
                return null;
            }

            return sources;
        });
}

async function getArtistImageLastFm(artist) {
    if (!artist || !artist.url) return null;
    console.log(`Fetching image for ${artist.name}`);
    
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

export async function getArtistImage(artist) {
    let artistInfo = await getArtistInfo(artist);
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

    // await download(albumImage, dest);
}