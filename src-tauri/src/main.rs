// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod db_refactor;

use rusqlite::Connection;
use std::fs;

// https://tauri.app/v1/guides/features/events/
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

struct MusicPlayer {
    sink: rodio::Sink
}

fn main() {
    init_database();
    init_audio_player();
}

fn init_database() {
    println!("Initializing database...");
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let schema = fs::read_to_string("db/schema.sql").unwrap();

    db.execute_batch(schema.as_str()).expect("Failed to create database");
}

fn init_audio_player() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .setup(|app| {
            // https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/stronghold
            let salt_path = app.path_resolver().app_local_data_dir().expect("Failed to get app data dir").join("salt.txt");

            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            Ok(())
        })
        .manage(MusicPlayer {
            sink: rodio::Sink::try_new(&stream_handle).unwrap()
        })
        .invoke_handler(tauri::generate_handler![
            write_file,
            audio::play,
            audio::add_to_queue,
            audio::pause,
            audio::resume,
            audio::stop,
            audio::seek,
            audio::skip_forward,
            audio::skip_backward,
            audio::set_volume,
            db_refactor::register_file,
            db_refactor::register_dir,
            db_refactor::get_all_albums,
            db_refactor::get_albums_by_artist,
            db_refactor::get_songs_by_album,
            db_refactor::get_all_artists,
            db_refactor::remove_album,
            db_refactor::remove_song,
            db_refactor::update_metadata_song,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn write_file(path: String, contents: Vec<u8>) {
    println!("Writing file to: {}", path);
    fs::write(path, contents);
}