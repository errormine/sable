import { invoke } from "@tauri-apps/api/core";
import { addToast } from "./stores/notifications";

export function sec2time(seconds) {
    let minutes = Math.floor(seconds / 60);
    let secs = Math.floor(seconds % 60);

    return `${minutes}:${secs < 10 ? '0' + secs : secs}`;
};

export async function invokeWithToast(func, args) {
    await invoke(func, args)
        .then(result => {
            addToast({
                message: result,
                type: 'success',
                dismissable: true,
                timeout: 3000
            })
        })
        .catch(err => {
            addToast({
                message: err,
                type: 'error',
                dismissable: true,
                timeout: 5000
            })
        });
}