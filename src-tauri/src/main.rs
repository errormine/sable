// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod db;

use rusqlite::Connection;
use std::fs;

// https://tauri.app/v1/guides/features/events/
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

struct MusicPlayer {
    sink: rodio::Sink,
}

fn main() {
    init_database();
    init_audio_player();
}

fn init_database() {
    println!("Initializing database...");
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let schema = fs::read_to_string("db/schema.sql").unwrap();

    db.execute_batch(schema.as_str())
        .expect("Failed to create database");
}

fn init_audio_player() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_stronghold::Builder::new(|password| {
            // https://v2.tauri.app/plugin/stronghold/
            use argon2::{hash_raw, Config, Variant, Version};

            let config = Config {
                lanes: 4,
                mem_cost: 10_000,
                time_cost: 10,
                variant: Variant::Argon2id,
                version: Version::Version13,
                ..Default::default()
            };
            let salt = "your-salt".as_bytes();
            let key =
                hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

            key.to_vec()
        }).build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .manage(MusicPlayer {
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
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
            db::register_dir,
            db::get_all_albums,
            db::get_albums_by_artist,
            db::get_all_songs,
            db::get_songs_by_album,
            db::get_all_artists,
            db::remove_album,
            db::remove_song,
            db::update_metadata_song,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn write_file(path: String, contents: Vec<u8>) {
    println!("Writing file to: {}", path);
    fs::write(path, contents);
}
