export function sec2time(seconds) {
    let minutes = Math.floor(seconds / 60);
    let secs = Math.floor(seconds % 60);

    return `${minutes}:${secs < 10 ? '0' + secs : secs}`;
};