// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use std::{fs, path::Path, sync::mpsc, thread};
use jwalk::WalkDir;
use rusqlite::{params, Connection, Result};
use serde_json::json;
use tauri::Manager;
use std::path::PathBuf;
use id3::{Tag, TagLike};

// https://tauri.app/v1/guides/features/events/
#[derive(Clone, serde::Serialize)]
struct Payload {
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
        .manage(MusicPlayer {
            sink: rodio::Sink::try_new(&stream_handle).unwrap()
        })
        .invoke_handler(tauri::generate_handler![
            audio::player::play,
            audio::player::pause,
            audio::player::resume,
            audio::player::seek,
            audio::player::skip_forward,
            audio::player::skip_backward,
            register_songs,
            get_albums,
            get_songs_by_album
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn register_songs(dir: &Path, app: tauri::AppHandle) {
    let dir = dir.to_path_buf();
    
    let register_songs = move || {
        let songs = get_song_files(&dir);
        let mut progress = 0;
        app.emit_all("total_songs", Payload { message: songs.len().to_string() }).unwrap();

        for song in songs {
            let db = Connection::open("D:/Documents/music.db").unwrap();
            let dir = song.parent().unwrap();
            let cover_path = dir.join("Cover.jpg");
            let song = song.to_str().unwrap();
            let tag = Tag::read_from_path(song);

            match tag {
                Ok(tag) => {
                    let title = tag.title();
                    let artist = tag.artist();
                    let album = tag.album();
                    let track_number = tag.track();
                    let disc_number = tag.disc();
                    let duration = audio::player::get_duration(song.to_string());
            
                    db.execute(
                        "INSERT OR IGNORE INTO album (title, artist, cover_path) VALUES (?1, ?2, ?3)",
                        params![album, artist, cover_path.to_str()]
                    ).unwrap();
            
                    db.execute(
                        "INSERT OR IGNORE INTO song (file_path, title, artist, album, track_number, disc_number, duration) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![song, title, artist, album, track_number, disc_number, duration]
                    ).unwrap();
                },
                Err(_) => continue
            }

            progress += 1;
            app.emit_all("songs_registered", Payload { message: progress.to_string() }).unwrap();
        };

        println!("Done registering songs");
        app.emit_all("songs_registered", Payload { message: "done".to_string() }).unwrap();
    };

    thread::spawn(register_songs);
}

#[tauri::command]
fn get_albums() -> String {
    let db = Connection::open("D:/Documents/music.db");

    match db {
        Ok(db) => {
            let mut stmt = db.prepare("SELECT title, artist, cover_path FROM album").unwrap();
            let mut rows = stmt.query(params![]).unwrap();
            
            let mut albums_json = Vec::new();
            while let Some(row) = rows.next().unwrap() {
                let title: String = row.get(0).unwrap();
                let artist: String = row.get(1).unwrap();
                let cover_path: String = row.get(2).unwrap();

                let album = json!({
                    "title": title,
                    "artist": artist,
                    "cover_path": cover_path
                });

                albums_json.push(album);
            }

            return json!(albums_json).to_string();
        },
        Err(_) => return String::from("[]")
    };
}

#[tauri::command]
fn get_songs_by_album(album: String, artist: String) -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let mut stmt = db.prepare("SELECT * FROM song WHERE album = ?1 AND artist = ?2").unwrap();
    let mut rows = stmt.query(params![album, artist]).unwrap();
    
    let mut songs_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let file_path: String = row.get(0).unwrap();
        let title: String = row.get(1).unwrap();
        let artist: String = row.get(2).unwrap();
        let album: String = row.get(3).unwrap();
        let track_number: i32 = row.get(4).unwrap_or(0);
        let disc_number: i32 = row.get(5).unwrap_or(0);
        let duration: i64 = row.get(6).unwrap();

        let song = json!({
            "file_path": file_path,
            "title": title,
            "artist": artist,
            "album": album,
            "track_number": track_number,
            "disc_number": disc_number,
            "duration": duration
        });

        songs_json.push(song);
    }

    return json!(songs_json).to_string();
}

fn get_song_files(dir: &Path) -> Vec<PathBuf> {
    let mut songs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(dir).sort(true) {
        let path = entry.unwrap().path();

        if path.is_file() {
            let extension = path.extension().unwrap();
            
            match extension.to_str().unwrap() {
                "mp3" => songs.push(path.to_path_buf()),
                _ => continue
            }
        }
    }

    return songs;
}