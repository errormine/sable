import { fs, http } from "@tauri-apps/api";
import { ResponseType } from "@tauri-apps/api/http";
import { invoke } from "@tauri-apps/api/tauri";

const ApiCredentials = {
    apiKey: import.meta.env.VITE_LASTFM_API_KEY,
    secret: import.meta.env.VITE_LASTFM_API_SECRET,
};

async function download(url, dest) {
    const client = await http.getClient();
    const response = await client.get(url, { responseType: ResponseType.Binary });

    await invoke("write_file", { path: dest, contents: response.data });
}

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

export async function getAlbumId(title, artist) {
    return http.fetch(`https://api.deezer.com/search/album?q=artist:"${artist}" album:"${title}"&limit=1`)
        .then(res => res.data)
        .then(data => {
            return data.data[0].id;
        });
}

export async function getAlbumDeezer(id) {
    return http.fetch(`https://api.deezer.com/album/${id}`)
        .then(res => res.data)
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