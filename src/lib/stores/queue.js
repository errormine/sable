import { writable } from 'svelte/store';

export const songQueue = writable([]);
export const currentSongIndex = writable(0);