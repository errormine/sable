import { http } from "@tauri-apps/api";
import { ResponseType } from "@tauri-apps/api/http";

const ApiCredentials = {
    apiKey: import.meta.env.VITE_LASTFM_API_KEY,
    secret: import.meta.env.VITE_LASTFM_API_SECRET,
};

export async function retrieveFromCache(name) {
    let cached = localStorage.getItem(name.toLowerCase());

    if (cached) {
        return JSON.parse(cached);
    }

    return null;
}

export async function getArtistInfo(name) {
    return fetch(`https://ws.audioscrobbler.com/2.0/?method=artist.getinfo&artist=${name}&api_key=${ApiCredentials.apiKey}&format=json`)
        .then(res => res.json())
        .then(async (data) => {
            let artist = data.artist;
            let thumbnail = await getArtistImageLastFm(artist);
            let combined = { ...artist, thumbnail }

            if (artist != null) {
                localStorage.setItem(artist.name.toLowerCase(), JSON.stringify(combined));
            }

            return combined;
        });
}

async function getArtistImageLastFm(artist) {
    if (!artist || !artist.url) return null;

    
    
    let lastFmImage = http.fetch(artist.url, {
            method: 'GET',
            responseType: ResponseType.Text,
            timeout: 10,
        })
        .then(res => {
            let parser = new DOMParser();
            let doc = parser.parseFromString(res.data, 'text/html');
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
    return http.fetch(`https://api.deezer.com/search/artist?q="${artist.name}"&limit=1&strict=on`)
        .then(res => {
            return res.data.data[0].picture_medium;
        });
}

export async function getArtistImage(name) {
    let cached = await retrieveFromCache(name);

    if (cached) {
        return cached.thumbnail;
    }

    let artistInfo = await getArtistInfo(name);

    return artistInfo.thumbnail;

}

export async function getArtistTags(name) {
    let cached = await retrieveFromCache(name);

    if (cached) {
        return cached.tags.tag;
    }

    let artistInfo = await getArtistInfo(name);

    return artistInfo.tags.tag;
}