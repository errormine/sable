// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod db;

use rusqlite::Connection;
use tauri::Manager;
use tauri_plugin_http::reqwest;
use std::{fs, path::Path};

// https://tauri.app/v1/guides/features/events/
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

struct MusicPlayer {
    sink: rodio::Sink,
}

#[tauri::command]
async fn download(url: String, dest: String, name: String) -> Result<String, String> {
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    let dir = Path::new(&dest);
    let path = dir.join(name);

    if !dir.exists() {
        fs::create_dir_all(&dest).map_err(|e| e.to_string())?
    }

    fs::write(path, bytes).map_err(|e| e.to_string())?;

    Ok("OK".into())
}

fn main() {
    init_audio_player();
}

fn init_audio_player() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .setup(|app| {
            let db_path = app.path().app_data_dir().unwrap().join("music.db");
            let conn = Connection::open(db_path).unwrap();
            let schema_path = app.path().resource_dir().unwrap().join("db/schema.sql");
            let schema = fs::read_to_string(schema_path).unwrap();

            conn.execute_batch(&schema).expect("Failed to create database");
            Ok(())
        })
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
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
                let salt = "dsgfdfgdfg345345".as_bytes();
                let key = hash_raw(password.as_ref(), salt, &config).expect("failed to hash password");

                key.to_vec()
            })
        .build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .manage(MusicPlayer {
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
        })
        .invoke_handler(tauri::generate_handler![
            download,
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