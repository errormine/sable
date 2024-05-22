use std::fs::File;
use std::io::BufReader;
use rodio::Source;

use crate::MusicPlayer;

fn get_source(file_path: String) -> rodio::Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(file_path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    return source;
}

pub fn get_duration(file_path: String) -> u64 {
    let source = get_source(file_path);
    let duration = source.total_duration();

    return match duration {
        Some(duration) => duration.as_secs(),
        None => 0,
    };
}

#[tauri::command]
pub fn play(file_path: String, state: tauri::State<MusicPlayer>) {
    let source = get_source(file_path);
    state.sink.stop();
    state.sink.append(source);
}

#[tauri::command]
pub fn pause(state: tauri::State<MusicPlayer>) {
    state.sink.pause();
}

#[tauri::command]
pub fn resume(state: tauri::State<MusicPlayer>) {
    state.sink.play();
}

#[tauri::command]
pub fn seek(position: u64, state: tauri::State<MusicPlayer>) {
    let duration = std::time::Duration::from_secs(position);
    state.sink.try_seek(duration);
}

#[tauri::command]
pub fn skip_forward(state: tauri::State<MusicPlayer>) {
    state.sink.skip_one();
}

#[tauri::command]
pub fn skip_backward(state: tauri::State<MusicPlayer>) {
    // TODO: Implement skipBackward
}