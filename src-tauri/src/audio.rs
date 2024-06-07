use rodio::Source;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::MusicPlayer;

pub fn get_source(file_path: &str) -> Result<rodio::Decoder<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    Ok(rodio::Decoder::new(buffer)?)
}

pub fn get_duration(file_path: &str) -> Result<u64, Box<dyn Error>> {
    let source = get_source(file_path)?;
    let duration = source.total_duration();

    return match duration {
        Some(d) => Ok(d.as_secs()),
        None => Err("Could not read song duration".into()),
    };
}

#[tauri::command]
pub fn play(file_path: String, state: tauri::State<MusicPlayer>) -> Result<String, String> {
    let source = get_source(file_path.as_str()).map_err(|e| e.to_string())?;
    state.sink.stop();
    state.sink.append(source);
    Ok("success".to_string())
}

#[tauri::command]
pub fn add_to_queue(file_path: String, state: tauri::State<MusicPlayer>) -> Result<String, String> {
    let source = get_source(file_path.as_str()).map_err(|e| e.to_string())?;
    state.sink.append(source);
    Ok("success".to_string())
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
pub fn stop(state: tauri::State<MusicPlayer>) {
    state.sink.stop();
}

#[tauri::command]
pub fn seek(position: String, state: tauri::State<MusicPlayer>) -> String {
    let position: u64 = position.parse().unwrap();
    let duration = std::time::Duration::from_secs(position);

    match state.sink.try_seek(duration) {
        Ok(_) => String::from("success"),
        Err(e) => format!("{}", e.to_string()),
    }
}

#[tauri::command]
pub fn skip_forward(state: tauri::State<MusicPlayer>) {
    state.sink.skip_one();
}

#[tauri::command]
pub fn skip_backward(state: tauri::State<MusicPlayer>) {
    // TODO: Implement skipBackward
}

#[tauri::command]
pub fn set_volume(volume: f32, state: tauri::State<MusicPlayer>) {
    // This is because the slider in the UI goes from 0 to 100... it explodes if it I make it go from 0 to 1
    let clamped = volume / 100.0;
    state.sink.set_volume(clamped);
}
