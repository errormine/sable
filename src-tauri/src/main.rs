// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use std::{error::Error, fs::{self, File, OpenOptions}, io::BufReader, path::Path, sync::{mpsc, Mutex}, thread::{self, scope}, time::Instant};
use jwalk::WalkDir;
use rodio::Source;
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
            audio::player::add_to_queue,
            audio::player::pause,
            audio::player::resume,
            audio::player::stop,
            audio::player::seek,
            audio::player::skip_forward,
            audio::player::skip_backward,
            audio::player::set_volume,
            register_file,
            register_dir,
            get_albums,
            get_songs_by_album,
            remove_album,
            remove_song,
            update_song_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn register_file(file_path: &Path, app: tauri::AppHandle) {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let dir = file_path.parent().unwrap();
    let cover_path = dir.join("Cover.jpg");
    let tag = Tag::read_from_path(file_path);

    match tag {
        Ok(tag) => {
            let song = &file_path.to_string_lossy().to_string();
            let title = tag.title();
            let artist = tag.artist();
            let album = tag.album();
            let track_number = tag.track();
            let disc_number = tag.disc();
            let duration = audio::player::get_duration(song);
    
            db.execute(
                "INSERT OR IGNORE INTO album (title, artist, cover_path) VALUES (?1, ?2, ?3)",
                params![album, artist, cover_path.to_str()]
            ).unwrap();
    
            db.execute(
                "INSERT OR IGNORE INTO song (file_path, title, artist, album, track_number, disc_number, duration) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![song, title, artist, album, track_number, disc_number, duration]
            ).unwrap();
        },
        Err(_) => return
    }

    app.emit_all("song_registered", Payload { message: "done".to_string() }).unwrap();
}

struct SongMetadata {
    file_path: String,
    cover_path: String,
    title: String,
    artist: String,
    album: String,
    track_number: u32,
    disc_number: u32,
    duration: u64
}

impl Clone for SongMetadata {
    fn clone(&self) -> Self {
        SongMetadata {
            file_path: self.file_path.clone(),
            cover_path: self.cover_path.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            album: self.album.clone(),
            track_number: self.track_number,
            disc_number: self.disc_number,
            duration: self.duration
        }
    }
}

fn get_metadata(songs: &Vec<PathBuf>, app: tauri::AppHandle) -> Vec<SongMetadata> {
    let songs_metadata = Mutex::new(Vec::new());
    let songs = songs.clone();

    let read_metadata = || {
        let mut progress = 0;

        for song in songs {
            let cover_path = song.clone().parent().unwrap().join("Cover.jpg").to_string_lossy().to_string();
            let tag = Tag::read_from_path(&song);
            
            match tag {
                Ok(tag) => {
                    let title = tag.title().unwrap().to_string();
                    let artist = tag.artist().unwrap().to_string();
                    let album = tag.album().unwrap().to_string();
                    let track_number = tag.track().unwrap_or(0);
                    let disc_number = tag.disc().unwrap_or(0);
                    let mut guard = songs_metadata.lock().unwrap();
                    let duration = audio::player::get_duration(&song.to_string_lossy().to_string());

                    let song_metadata = SongMetadata {
                        file_path: song.to_string_lossy().to_string(),
                        cover_path,
                        title,
                        artist,
                        album,
                        track_number,
                        disc_number,
                        duration
                    };

                    guard.push(song_metadata);
                    progress += 1;
                    app.emit_all("songs_registered", Payload { message: progress.to_string() }).unwrap();
                },
                Err(_) => continue
            }
        }
    };

    scope(|s| {
        s.spawn(read_metadata);
    });

    return songs_metadata.into_inner().unwrap();
}

#[tauri::command]
fn register_dir(dir: &Path, app: tauri::AppHandle) {
    let dir = dir.to_path_buf();
    let now = Instant::now();
    
    thread::spawn(move || {
        let songs = get_song_files(&dir);
        app.emit_all("total_songs", Payload { message: songs.len().to_string() }).unwrap();
        let songs_metadata = get_metadata(&songs, app.clone());

        let mut db = Connection::open("D:/Documents/music.db").unwrap();
        let tx = db.transaction().unwrap();

        for song in songs_metadata {
            tx.execute(
                "INSERT OR IGNORE INTO album (title, artist, cover_path) VALUES (?1, ?2, ?3)",
                params![&song.album, &song.artist, &song.cover_path]
            ).unwrap();

            tx.execute(
                "INSERT OR IGNORE INTO song (file_path, title, artist, album, track_number, disc_number, duration) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![&song.file_path, &song.title, &song.artist, &song.album, &song.track_number, &song.disc_number, &song.duration]
            ).unwrap();
        };

        tx.commit().unwrap();

        println!("Done registering songs");
        app.emit_all("songs_registered", Payload { message: "done".to_string() }).unwrap();
        println!("Elapsed time: {:?}", now.elapsed());
    });
}

#[tauri::command]
fn get_albums() -> String {
    let db = Connection::open("D:/Documents/music.db");
    println!("{}", "Fetching albums...");
    let now = Instant::now();

    match db {
        Ok(db) => {
            let mut stmt = db.prepare("SELECT title, artist, cover_path FROM album ORDER BY artist, title").unwrap();
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

            println!("Elapsed time: {:?}", now.elapsed());
            return json!(albums_json).to_string();
        },
        Err(_) => return String::from("[]")
    };
}

#[tauri::command]
fn get_songs_by_album(title: String, artist: String) -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let cover_path: String = db.query_row("SELECT cover_path FROM album WHERE title = ?1 AND artist = ?2", params![title, artist], |row| row.get(0)).unwrap();
    let mut stmt = db.prepare("SELECT * FROM song WHERE album = ?1 AND artist = ?2 ORDER BY disc_number, track_number").unwrap();
    let mut rows = stmt.query(params![title, artist]).unwrap();
    
    let mut songs_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let file_path: String = row.get(0).unwrap();
        let title: String = row.get(1).unwrap();
        let artist: String = row.get(2).unwrap();
        let album: String = row.get(3).unwrap();
        let track_number: u32 = row.get(4).unwrap_or(0);
        let disc_number: u32 = row.get(5).unwrap_or(0);
        let duration: u32 = row.get(6).unwrap_or(0);

        let song = json!({
            "file_path": file_path,
            "cover_path": cover_path,
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

#[tauri::command]
fn remove_album(album: String, artist: String) {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    db.execute("DELETE FROM song WHERE album = ?1 AND artist = ?2", params![album, artist]).unwrap();
    db.execute("DELETE FROM album WHERE title = ?1 AND artist = ?2", params![album, artist]).unwrap();
}

#[tauri::command]
fn remove_song(file_path: String) {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    db.execute("DELETE FROM song WHERE file_path = ?1", params![file_path]).unwrap();
}

#[tauri::command]
fn update_song_info(file_path: String, title: String, artist: String, album: String) -> String {
    let result = update_metadata(&file_path, title, artist, album);
    
    match result {
        Ok(_) => {
            remove_song(file_path);
            return String::from("success");
        },
        Err(e) => {
            println!("{}", e);
            return format!("Error updating metadata: {}", e);
        }
    };
}

fn update_metadata(file_path: &str, title: String, artist: String, album: String) -> Result<(), Box<dyn Error>> {
    let mut tag = Tag::read_from_path(&file_path)?;

    tag.set_title(title);
    tag.set_artist(artist);
    tag.set_album(album);
    // tag.set_track(track_number);
    // tag.set_disc(disc_number);
    tag.write_to_path(file_path, tag.version()).unwrap();

    Ok(())
}