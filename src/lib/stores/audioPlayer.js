import { get, writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { getSession, lastFm, lastFmConnected } from './lastfmAPI';

export const currentSong = writable({
    file_path: '',
    cover_path: '',
    title: '',
    artist: '',
    album_title: '',
    album_artist: '',
    track_number: 0,
    duration: 0 
});
export const songProgress = writable(0);
export const isPlaying = writable(false);
export const startedPlayingAt = writable(0);

let intervalIndex;

export async function play(song) {
    if (!song.file_path) return;

    if (get(currentSong) != song) {
        await invoke('play', { filePath: song.file_path });
        songProgress.set(0);
        currentSong.set(song);
        startedPlayingAt.set(Math.floor(Date.now() / 1000));
        if (get(lastFmConnected)) {
            let session = await getSession();
            lastFm.track.updateNowPlaying({
                artist: song.artist,
                track: song.title,
                album: song.album_title,
                albumArtist: song.album_artist,
                trackNumber: song.track_number,
                duration: song.duration,
            }, session.key)
            .then(res => console.log(res));
        };
    }

    beginPlayBack();
}

export async function togglePlayback() {
    if (get(currentSong).file_path == '') return;
    get(isPlaying) ? pausePlayback() : beginPlayBack();
}

export async function beginPlayBack() {
    if (get(isPlaying)) return;
    clearInterval(intervalIndex);
    await invoke('resume');
    intervalIndex = setInterval(async () => {
        songProgress.update((n) => n + 1);
    }, 1000);
    isPlaying.set(true);
}

export async function pausePlayback() {
    await invoke('pause');
    clearInterval(intervalIndex);
    isPlaying.set(false);
}

export async function stopPlayback() {
    await invoke('stop');
    clearInterval(intervalIndex);
    isPlaying.set(false);
}

export const songQueue = writable([]);
export const currentSongIndex = writable(0);

export function setQueue(songs, offset = 0) {
    songQueue.set(songs);
    currentSongIndex.set(offset);
}

export function insertIntoQueue(songs) {
    songQueue.update((queue) => {
        queue.splice(get(currentSongIndex) + 1, 0, songs);
        console.log(queue);
        return queue;
    });
}

export async function addToQueue(songs) {
    songQueue.update((queue) => queue.concat(songs));
}

export async function attemptPlayNext() {
    currentSongIndex.update((n) => n + 1);
    let nextSong = get(songQueue)[get(currentSongIndex)];
    if (!nextSong) return;
    play(nextSong);
}

export async function attemptPlayPrevious() {
    currentSongIndex.update((n) => n - 1);
    let previousSong = get(songQueue)[get(currentSongIndex)];
    if (!previousSong) return;
    play(previousSong);
}

export async function jumpToSong(index) {
    currentSongIndex.set(index);
    let song = get(songQueue)[index];
    if (!song) return;
    play(song);
}

export const loopMode = writable(false);
export const shuffleMode = writable(false);

export async function toggleLoopMode() {
    loopMode.update((n) => !n);
}

export async function toggleShuffleMode() {
    shuffleMode.update((n) => !n);
}