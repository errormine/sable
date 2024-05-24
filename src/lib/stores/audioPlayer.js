import { get, writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export const currentSong = writable({
    title: '',
    artist: '',
    album: '',
    file_path: '',
    cover_path: '',
    duration: 0 
});
export const songProgress = writable(0);
export const isPlaying = writable(false);

let intervalIndex;

export async function play(song) {
    if (!song.file_path) return;

    if (get(currentSong) != song) {
        console.log('Playing new song: ', song.title);
        await invoke('play', { filePath: song.file_path });
        songProgress.set(0);
        currentSong.set(song);
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

export function setQueue(songs, offset) {
    songQueue.set(songs);
    currentSongIndex.set(offset - 1);
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